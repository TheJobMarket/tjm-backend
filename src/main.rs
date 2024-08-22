use crate::api::api_routes;
use crate::db::Database;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

pub(crate) mod api;
pub(crate) mod db;
pub(crate) mod models;
pub(crate) mod schema;
pub(crate) mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Data::new(Database::new());
    // Port on which API requests should be made.
    let port = 80;

    println!("Listening on port {port}");

    HttpServer::new(move || App::new().service(api_routes()).app_data(db.clone()))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
