#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

use crate::api::method::query_response::QueryResponse;
use crate::api::method::query_response::QueryResult;
use crate::api::param::from_json;
use crate::api::param::from_value;
use crate::api::param::DbResponse;
use crate::api::param::Param;
use crate::api::protocol::Status;
use crate::api::ErrorKind;
use crate::api::Method;
use crate::api::Result;
use crate::api::Route;
#[cfg(not(target_arch = "wasm32"))]
use futures::TryStreamExt;
use indexmap::IndexMap;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
#[cfg(not(target_arch = "wasm32"))]
use reqwest::header::ACCEPT;
#[cfg(not(target_arch = "wasm32"))]
use reqwest::header::CONTENT_TYPE;
use reqwest::RequestBuilder;
use serde::Deserialize;
use serde::Serialize;
use std::mem;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
use surrealdb::sql::Array;
use surrealdb::sql::Strand;
use surrealdb::sql::Value;
#[cfg(not(target_arch = "wasm32"))]
use tokio::fs::OpenOptions;
#[cfg(not(target_arch = "wasm32"))]
use tokio::io;
#[cfg(not(target_arch = "wasm32"))]
use tokio::io::AsyncReadExt;
#[cfg(not(target_arch = "wasm32"))]
use tokio_util::compat::FuturesAsyncReadCompatExt;
use url::Url;

type HttpRoute = Route<(Method, Param), Result<DbResponse>>;

const SQL_PATH: &str = "sql";

/// An HTTP client for communicating with the server via HTTP
#[derive(Debug, Clone)]
pub struct Client {
	method: Method,
}

enum Auth {
	Basic {
		user: String,
		pass: String,
	},
	Bearer {
		token: String,
	},
}

trait Authenticate {
	fn auth(self, auth: &Option<Auth>) -> Self;
}

impl Authenticate for RequestBuilder {
	fn auth(self, auth: &Option<Auth>) -> Self {
		match auth {
			Some(Auth::Basic {
				user,
				pass,
			}) => self.basic_auth(user, Some(pass)),
			Some(Auth::Bearer {
				token,
			}) => self.bearer_auth(token),
			None => self,
		}
	}
}

#[derive(Debug, Deserialize)]
struct HttpQueryResponse {
	status: Status,
	result: Option<serde_json::Value>,
	detail: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Root {
	user: String,
	pass: String,
}

#[derive(Debug, Deserialize)]
struct AuthResponse {
	token: Option<String>,
}

async fn submit_auth(request: RequestBuilder) -> Result<Value> {
	let response = request.send().await?.error_for_status()?;
	let text = response.text().await?;
	tracing::info!("Response {text}");
	let response: AuthResponse = serde_json::from_str(&text)?;
	Ok(response.token.filter(|token| token != "NONE").into())
}

async fn query(request: RequestBuilder) -> Result<QueryResponse> {
	tracing::info!("{request:?}");
	let response = request.send().await?.error_for_status()?;
	let text = response.text().await?;
	tracing::info!("Response {text}");
	let responses: Vec<HttpQueryResponse> = serde_json::from_str(&text)?;
	let mut vec: Vec<QueryResult> = Vec::with_capacity(responses.len());
	for response in responses {
		match response.status {
			Status::Ok => {
				if let Some(value) = response.result {
					match from_json(value) {
						Value::Array(Array(array)) => vec.push(Ok(array).into()),
						Value::None | Value::Null => vec.push(Ok(vec![]).into()),
						value => vec.push(Ok(vec![value]).into()),
					}
				}
			}
			Status::Err => {
				if let Some(error) = response.detail {
					vec.push(Err(ErrorKind::Query.with_context(error)).into());
				}
			}
		}
	}

	Ok(vec.into())
}

async fn take(one: bool, request: RequestBuilder) -> Result<Value> {
	if let Some(result) = query(request).await?.into_inner().pop() {
		let mut vec = result.into_inner()?;
		match one {
			true => match vec.pop() {
				Some(Value::Array(Array(mut vec))) => {
					if let [value] = &mut vec[..] {
						return Ok(mem::take(value));
					}
				}
				Some(Value::None | Value::Null) | None => {}
				Some(value) => {
					return Ok(value);
				}
			},
			false => {
				return Ok(Value::Array(Array(vec)));
			}
		}
	}
	match one {
		true => Ok(Value::None),
		false => Ok(Value::Array(Array(vec![]))),
	}
}

#[cfg(not(target_arch = "wasm32"))]
async fn export(request: RequestBuilder, file: PathBuf) -> Result<Value> {
	let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(file).await?;
	let mut response = request
		.send()
		.await?
		.error_for_status()?
		.bytes_stream()
		.map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
		.into_async_read()
		.compat();
	io::copy(&mut response, &mut file).await?;
	Ok(Value::None)
}

#[cfg(not(target_arch = "wasm32"))]
async fn import(request: RequestBuilder, file: PathBuf) -> Result<Value> {
	let mut file = OpenOptions::new().read(true).open(file).await?;
	let mut contents = vec![];
	file.read_to_end(&mut contents).await?;
	// ideally we should pass `file` directly into the body
	// but currently that results in
	// "HTTP status client error (405 Method Not Allowed) for url"
	request.body(contents).send().await?.error_for_status()?;
	Ok(Value::None)
}

async fn version(request: RequestBuilder) -> Result<Value> {
	let response = request.send().await?.error_for_status()?;
	let version = response.text().await?;
	Ok(version.into())
}

async fn health(request: RequestBuilder) -> Result<Value> {
	request.send().await?.error_for_status()?;
	Ok(Value::None)
}

async fn router(
	(method, param): (Method, Param),
	base_url: &Url,
	client: &reqwest::Client,
	headers: &mut HeaderMap,
	vars: &mut IndexMap<String, String>,
	auth: &mut Option<Auth>,
) -> Result<DbResponse> {
	let mut params = match param.query {
		Some((query, bindings)) => {
			vec![query.to_string().into(), bindings.into()]
		}
		None => param.other,
	};

	match method {
		Method::Use => {
			let path = base_url.join(SQL_PATH)?;
			let (ns, db) = match &mut params[..] {
				[Value::Strand(Strand(ns)), Value::Strand(Strand(db))] => {
					(mem::take(ns), mem::take(db))
				}
				_ => unreachable!(),
			};
			let request = client
				.post(path)
				.headers(headers.clone())
				.header("NS", &ns)
				.header("DB", &db)
				.auth(&auth)
				.body("RETURN true");
			take(true, request).await?;
			headers.insert("NS", HeaderValue::from_str(&ns)?);
			headers.insert("DB", HeaderValue::from_str(&db)?);
			Ok(DbResponse::Other(Value::None))
		}
		Method::Signin => {
			let path = base_url.join(Method::Signin.as_str())?;
			let credentials = match &params[..] {
				[credentials] => serde_json::to_string(credentials)?,
				_ => unreachable!(),
			};
			let request = client.post(path).headers(headers.clone()).auth(&auth).body(credentials);
			let value = submit_auth(request).await?;
			if let [credentials] = &params[..] {
				if let Ok(Root {
					user,
					pass,
				}) = from_value(credentials)
				{
					*auth = Some(Auth::Basic {
						user,
						pass,
					});
				} else {
					*auth = Some(Auth::Bearer {
						token: value.to_strand().as_string(),
					});
				}
			}
			Ok(DbResponse::Other(value))
		}
		Method::Signup => {
			let path = base_url.join(Method::Signup.as_str())?;
			let credentials = match &params[..] {
				[credentials] => serde_json::to_string(credentials)?,
				_ => unreachable!(),
			};
			let request = client.post(path).headers(headers.clone()).auth(&auth).body(credentials);
			let value = submit_auth(request).await?;
			Ok(DbResponse::Other(value))
		}
		Method::Authenticate => {
			let path = base_url.join(SQL_PATH)?;
			let token = match &mut params[..1] {
				[Value::Strand(Strand(token))] => mem::take(token),
				_ => unreachable!(),
			};
			let request =
				client.post(path).headers(headers.clone()).bearer_auth(&token).body("RETURN true");
			take(true, request).await?;
			*auth = Some(Auth::Bearer {
				token,
			});
			Ok(DbResponse::Other(Value::None))
		}
		Method::Invalidate => {
			*auth = None;
			Ok(DbResponse::Other(Value::None))
		}
		Method::Create => {
			let path = base_url.join(SQL_PATH)?;
			let statement = crate::api::create_statement(&mut params);
			let request =
				client.post(path).headers(headers.clone()).auth(&auth).body(statement.to_string());
			let value = take(true, request).await?;
			Ok(DbResponse::Other(value))
		}
		Method::Update => {
			let path = base_url.join(SQL_PATH)?;
			let (one, statement) = crate::api::update_statement(&mut params);
			let request =
				client.post(path).headers(headers.clone()).auth(&auth).body(statement.to_string());
			let value = take(one, request).await?;
			Ok(DbResponse::Other(value))
		}
		Method::Patch => {
			let path = base_url.join(SQL_PATH)?;
			let (one, statement) = crate::api::patch_statement(&mut params);
			let request =
				client.post(path).headers(headers.clone()).auth(&auth).body(statement.to_string());
			let value = take(one, request).await?;
			Ok(DbResponse::Other(value))
		}
		Method::Merge => {
			let path = base_url.join(SQL_PATH)?;
			let (one, statement) = crate::api::merge_statement(&mut params);
			let request =
				client.post(path).headers(headers.clone()).auth(&auth).body(statement.to_string());
			let value = take(one, request).await?;
			Ok(DbResponse::Other(value))
		}
		Method::Select => {
			let path = base_url.join(SQL_PATH)?;
			let (one, statement) = crate::api::select_statement(&mut params);
			let request =
				client.post(path).headers(headers.clone()).auth(&auth).body(statement.to_string());
			let value = take(one, request).await?;
			Ok(DbResponse::Other(value))
		}
		Method::Delete => {
			let path = base_url.join(SQL_PATH)?;
			let statement = crate::api::delete_statement(&mut params);
			let request =
				client.post(path).headers(headers.clone()).auth(&auth).body(statement.to_string());
			let value = take(true, request).await?;
			Ok(DbResponse::Other(value))
		}
		Method::Query => {
			let path = base_url.join(SQL_PATH).unwrap();
			let mut request = client.post(path).headers(headers.clone()).query(&vars).auth(&auth);
			match &mut params[..] {
				[Value::Strand(Strand(statements))] => {
					request = request.body(mem::take(statements));
				}
				[Value::Strand(Strand(statements)), Value::Object(bindings)] => {
					let bindings: Vec<_> =
						bindings.iter().map(|(key, value)| (key, value.to_string())).collect();
					request = request.query(&bindings).body(mem::take(statements));
				}
				_ => unreachable!(),
			}
			let values = query(request).await?;
			Ok(DbResponse::Query(values))
		}
		#[cfg(target_arch = "wasm32")]
		Method::Export | Method::Import => unreachable!(),
		#[cfg(not(target_arch = "wasm32"))]
		Method::Export => {
			let path = base_url.join(Method::Export.as_str()).unwrap();
			let file = param.file.expect("file to export into");
			let request = client
				.get(path)
				.headers(headers.clone())
				.auth(&auth)
				.header(ACCEPT, "application/octet-stream");
			let value = export(request, file).await?;
			Ok(DbResponse::Other(value))
		}
		#[cfg(not(target_arch = "wasm32"))]
		Method::Import => {
			let path = base_url.join(Method::Import.as_str()).unwrap();
			let file = param.file.expect("file to import from");
			let request = client
				.post(path)
				.headers(headers.clone())
				.auth(&auth)
				.header(CONTENT_TYPE, "application/octet-stream");
			let value = import(request, file).await?;
			Ok(DbResponse::Other(value))
		}
		Method::Health => {
			let path = base_url.join(Method::Health.as_str()).unwrap();
			let request = client.get(path);
			let value = health(request).await?;
			Ok(DbResponse::Other(value))
		}
		Method::Version => {
			let path = base_url.join(method.as_str()).unwrap();
			let request = client.get(path);
			let value = version(request).await?;
			Ok(DbResponse::Other(value))
		}
		Method::Set => {
			let path = base_url.join(SQL_PATH)?;
			let (key, value) = match &mut params[..2] {
				[Value::Strand(Strand(key)), value] => (mem::take(key), value.to_string()),
				_ => unreachable!(),
			};
			let request = client
				.post(path)
				.headers(headers.clone())
				.auth(&auth)
				.query(&[(key.as_str(), value.as_str())])
				.body(format!("RETURN ${key}"));
			take(true, request).await?;
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
			let path = base_url.join(SQL_PATH)?;
			let table = match &params[..] {
				[table] => table.to_string(),
				_ => unreachable!(),
			};
			let request = client
				.post(path)
				.headers(headers.clone())
				.auth(&auth)
				.query(&[("table", table)])
				.body("LIVE SELECT * FROM type::table($table)");
			let value = take(true, request).await?;
			Ok(DbResponse::Other(value))
		}
		Method::Kill => {
			let path = base_url.join(SQL_PATH)?;
			let id = match &params[..] {
				[id] => id.to_string(),
				_ => unreachable!(),
			};
			let request = client
				.post(path)
				.headers(headers.clone())
				.auth(&auth)
				.query(&[("id", id)])
				.body("KILL type::string($id)");
			let value = take(true, request).await?;
			Ok(DbResponse::Other(value))
		}
	}
}
