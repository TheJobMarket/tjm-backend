use crate::api::api_routes;
use crate::db::Database;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

pub(crate) mod api;
pub(crate) mod db;
pub(crate) mod models;
pub(crate) mod schema;
pub(crate) mod utils;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = Data::new(Database::new());
    // Port on which API requests should be made.
    let port = 80;

    println!("Listening on port {port}");

    HttpServer::new(move || App::new().service(api_routes()).app_data(state.clone()))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
