use actix_web::{App, HttpServer};
use actix_web::web::Data;
use crate::api::api_routes;
use crate::db::Database;

mod api;
mod db;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Data::new(Database::new());

    HttpServer::new(move || {
        App::new()
            .service(api_routes())
            .app_data(db.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
