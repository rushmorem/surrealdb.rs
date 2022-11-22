//! Database storage engines

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

use crate::param::DbResponse;
use crate::param::Param;
use crate::ErrorKind;
use crate::Method;
use crate::Result;
use crate::Route;
use std::collections::BTreeMap;
use std::mem;
#[cfg(not(target_arch = "wasm32"))]
use surrealdb::channel;
use surrealdb::sql::Array;
use surrealdb::sql::Query;
use surrealdb::sql::Statement;
use surrealdb::sql::Statements;
use surrealdb::sql::Strand;
use surrealdb::sql::Value;
use surrealdb::Datastore;
use surrealdb::Response;
use surrealdb::Session;
#[cfg(not(target_arch = "wasm32"))]
use tokio::fs::OpenOptions;
#[cfg(not(target_arch = "wasm32"))]
use tokio::io;
#[cfg(not(target_arch = "wasm32"))]
use tokio::io::AsyncReadExt;
#[cfg(not(target_arch = "wasm32"))]
use tokio::io::AsyncWriteExt;

type DbRoute = Route<(Method, Param), Result<DbResponse>>;

/// In-memory database
#[cfg(feature = "mem")]
#[cfg_attr(docsrs, doc(cfg(feature = "mem")))]
#[derive(Debug)]
pub struct Mem;

/// File database
#[cfg(feature = "rocksdb")]
#[cfg_attr(docsrs, doc(cfg(feature = "rocksdb")))]
#[derive(Debug)]
pub struct File;

/// RocksDB database
#[cfg(feature = "rocksdb")]
#[cfg_attr(docsrs, doc(cfg(feature = "rocksdb")))]
#[derive(Debug)]
pub struct RocksDb;

/// IndxDB database
#[cfg(feature = "indxdb")]
#[cfg_attr(docsrs, doc(cfg(feature = "indxdb")))]
#[derive(Debug)]
pub struct IndxDb;

/// TiKV database
#[cfg(feature = "tikv")]
#[cfg_attr(docsrs, doc(cfg(feature = "tikv")))]
#[derive(Debug)]
pub struct TiKv;

/// FoundationDB database
#[cfg(feature = "fdb")]
#[cfg_attr(docsrs, doc(cfg(feature = "fdb")))]
#[derive(Debug)]
pub struct FDb;

fn process(responses: Vec<Response>) -> Vec<Result<Vec<Value>>> {
    let mut vec = Vec::with_capacity(responses.len());
    for response in responses {
        match response.result {
            Ok(value) => match value {
                Value::Array(Array(array)) => vec.push(Ok(array)),
                Value::None | Value::Null => vec.push(Ok(vec![])),
                value => vec.push(Ok(vec![value])),
            },
            Err(error) => vec.push(Err(ErrorKind::Query.with_context(error))),
        }
    }
    vec
}

async fn take(one: bool, responses: Vec<Response>) -> Result<Value> {
    if let Some(result) = process(responses).pop() {
        let mut vec = result?;
        match vec.pop() {
            Some(Value::Array(Array(mut vec))) => {
                if one {
                    if let [value] = &mut vec[..] {
                        return Ok(mem::take(value));
                    }
                } else {
                    return Ok(Value::Array(Array(vec)));
                }
            }
            Some(Value::None | Value::Null) | None => {}
            Some(value) => {
                return Ok(value);
            }
        }
    }
    match one {
        true => Ok(Value::None),
        false => Ok(Value::Array(Array(vec![]))),
    }
}

async fn router(
    (method, param): (Method, Param),
    #[cfg(target_arch = "wasm32")] kvs: &Datastore,
    #[cfg(not(target_arch = "wasm32"))] kvs: &'static Datastore,
    session: &mut Session,
    vars: &mut BTreeMap<String, Value>,
    strict: bool,
) -> Result<DbResponse> {
    let mut params = param.other;

    match method {
        Method::Use => {
            let (ns, db) = match &mut params[..] {
                [Value::Strand(Strand(ns)), Value::Strand(Strand(db))] => {
                    (mem::take(ns), mem::take(db))
                }
                _ => unreachable!(),
            };
            session.ns = Some(ns);
            session.db = Some(db);
            Ok(DbResponse::Other(Value::None))
        }
        Method::Signin | Method::Signup | Method::Authenticate | Method::Invalidate => {
            unreachable!()
        }
        Method::Create => {
            let statement = crate::create_statement(&mut params);
            let query = Query(Statements(vec![Statement::Create(statement)]));
            let response = kvs
                .process(query, &*session, Some(vars.clone()), strict)
                .await?;
            let value = take(true, response).await?;
            Ok(DbResponse::Other(value))
        }
        Method::Update => {
            let (one, statement) = crate::update_statement(&mut params);
            let query = Query(Statements(vec![Statement::Update(statement)]));
            let response = kvs
                .process(query, &*session, Some(vars.clone()), strict)
                .await?;
            let value = take(one, response).await?;
            Ok(DbResponse::Other(value))
        }
        Method::Patch => {
            let (one, statement) = crate::patch_statement(&mut params);
            let query = Query(Statements(vec![Statement::Update(statement)]));
            let response = kvs
                .process(query, &*session, Some(vars.clone()), strict)
                .await?;
            let value = take(one, response).await?;
            Ok(DbResponse::Other(value))
        }
        Method::Merge => {
            let (one, statement) = crate::merge_statement(&mut params);
            let query = Query(Statements(vec![Statement::Update(statement)]));
            let response = kvs
                .process(query, &*session, Some(vars.clone()), strict)
                .await?;
            let value = take(one, response).await?;
            Ok(DbResponse::Other(value))
        }
        Method::Select => {
            let (one, statement) = crate::select_statement(&mut params);
            let query = Query(Statements(vec![Statement::Select(statement)]));
            let response = kvs
                .process(query, &*session, Some(vars.clone()), strict)
                .await?;
            let value = take(one, response).await?;
            Ok(DbResponse::Other(value))
        }
        Method::Delete => {
            let statement = crate::delete_statement(&mut params);
            let query = Query(Statements(vec![Statement::Delete(statement)]));
            let response = kvs
                .process(query, &*session, Some(vars.clone()), strict)
                .await?;
            let value = take(true, response).await?;
            Ok(DbResponse::Other(value))
        }
        Method::Query => {
            let response = match param.query {
                Some((query, mut bindings)) => {
                    let mut vars = vars.clone();
                    vars.append(&mut bindings);
                    kvs.process(query, &*session, Some(vars), strict).await?
                }
                None => unreachable!(),
            };
            let values = process(response);
            Ok(DbResponse::Query(values))
        }
        #[cfg(target_arch = "wasm32")]
        Method::Export | Method::Import => unreachable!(),
        #[cfg(not(target_arch = "wasm32"))]
        Method::Export => {
            let file = param.file.expect("file to export into");
            let (tx, rx) = channel::new(1);
            let ns = session.ns.clone().unwrap_or_default();
            let db = session.db.clone().unwrap_or_default();
            tokio::spawn(async move {
                if let Err(error) = kvs.export(ns, db, tx).await {
                    tracing::error!("{error}");
                }
            });
            let (mut writer, mut reader) = io::duplex(10_240);
            tokio::spawn(async move {
                while let Ok(value) = rx.recv().await {
                    if let Err(error) = writer.write_all(&value).await {
                        tracing::error!("{error}");
                    }
                }
            });
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file)
                .await?;
            io::copy(&mut reader, &mut file).await?;
            Ok(DbResponse::Other(Value::None))
        }
        #[cfg(not(target_arch = "wasm32"))]
        Method::Import => {
            let file = param.file.expect("file to import from");
            let mut file = OpenOptions::new().read(true).open(file).await?;
            let mut statements = String::new();
            file.read_to_string(&mut statements).await?;
            let responses = kvs
                .execute(&statements, &*session, Some(vars.clone()), strict)
                .await?;
            for response in responses {
                response.result?;
            }
            Ok(DbResponse::Other(Value::None))
        }
        Method::Health => Ok(DbResponse::Other(Value::None)),
        Method::Version => Ok(DbResponse::Other(surrealdb::VERSION.into())),
        Method::Set => {
            let (key, value) = match &mut params[..2] {
                [Value::Strand(Strand(key)), value] => (mem::take(key), mem::take(value)),
                _ => unreachable!(),
            };
            vars.insert(key, value);
            Ok(DbResponse::Other(Value::None))
        }
        Method::Unset => {
            if let [Value::Strand(Strand(key))] = &params[..1] {
                vars.remove(key);
            }
            Ok(DbResponse::Other(Value::None))
        }
        Method::Live => {
            let table = match &mut params[..] {
                [value] => mem::take(value),
                _ => unreachable!(),
            };
            let mut vars = BTreeMap::new();
            vars.insert("table".to_owned(), table);
            let response = kvs
                .execute(
                    "LIVE SELECT * FROM type::table($table)",
                    &*session,
                    Some(vars),
                    strict,
                )
                .await?;
            let value = take(true, response).await?;
            Ok(DbResponse::Other(value))
        }
        Method::Kill => {
            let id = match &mut params[..] {
                [value] => mem::take(value),
                _ => unreachable!(),
            };
            let mut vars = BTreeMap::new();
            vars.insert("id".to_owned(), id);
            let response = kvs
                .execute("KILL type::string($id)", &*session, Some(vars), strict)
                .await?;
            let value = take(true, response).await?;
            Ok(DbResponse::Other(value))
        }
    }
}
