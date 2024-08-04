use crate::api::api_routes;
use crate::db::Database;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use crate::utils::generate_url_id;

mod api;
mod db;
mod models;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Data::new(Database::new());
    // Port on which API requests should be made.
    let port = 8080;

    println!("Listening on port {port}");

    HttpServer::new(move || App::new().service(api_routes()).app_data(db.clone()))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
