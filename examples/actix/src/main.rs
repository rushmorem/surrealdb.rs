mod error;
mod person;

use actix_web::{App, HttpServer};
use surrealdb_rs::net::WsClient;
use surrealdb_rs::param::Root;
use surrealdb_rs::protocol::Ws;
use surrealdb_rs::StaticClient;
use surrealdb_rs::Surreal;

static DB: Surreal<WsClient> = Surreal::new();

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	tracing_subscriber::fmt::init();

	DB.connect::<Ws>("localhost:8000").await?;

	DB.signin(Root {
		username: "root",
		password: "root",
	})
	.await?;

	DB.use_ns("namespace").use_db("database").await?;

	HttpServer::new(|| {
		App::new()
			.service(person::create)
			.service(person::read)
			.service(person::update)
			.service(person::delete)
			.service(person::list)
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await?;

	Ok(())
}
