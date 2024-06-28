use std::sync::Arc;
use actix_web::{App, HttpServer, Responder};
use actix_web::web::Data;
use crate::api::api_routes;
use crate::db::Database;
// use env_logger::Env;

mod api;
mod db;
/// Structs that strictly represent DB relations.
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(Env::default().default_filter_or("info"));
    let db = Data::new(Database::new());

    HttpServer::new(move || {
        // let db = Arc::clone(&db);
        App::new()
            .service(api_routes())
            .app_data(db.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
