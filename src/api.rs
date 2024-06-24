use std::sync::Mutex;
use actix_web::{get, HttpResponse, post, Responder, Scope, web};
use actix_web::web::Data;
use anyhow::Error;
use serde::{Deserialize, Serialize};
use crate::db::Database;
use crate::models::Post;

pub fn api_routes() -> Scope {
    web::scope("")
        .service(hello)
        .service(echo)
        .service(create_post)
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/posts")]
async fn create_post(db: Data<Database>, post: web::Json<Post>) -> impl Responder {
    println!("Post {:?}", post);
    match db.create_post(post.into_inner()) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}