use crate::db::Database;
use crate::models::{CompanyReq, JobReq};
use actix_web::web::Data;
use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use actix_web::guard::Head;
use actix_web::http::header::{ACCESS_CONTROL_ALLOW_ORIGIN, HeaderName};
use env_logger::builder;

pub fn api_routes() -> Scope {
    web::scope("")
        .service(hello)
        .service(echo)
        .service(get_jobs)
        .service(get_job_by_id)
        .service(create_job)
        .service(get_companies)
        .service(create_company)
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/jobs")]
async fn get_jobs(db: Data<Database>) -> impl Responder {
    match db.get_jobs() {
        Ok(jobs) => {
            HttpResponse::Ok()
                // TODO reduce origins allowed
                .insert_header((HeaderName::from(ACCESS_CONTROL_ALLOW_ORIGIN), "*"))
                .json(jobs)
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/jobs/{id}")]
async fn get_job_by_id(db: Data<Database>, job_id: web::Path<i32>) -> impl Responder {
    match db.find_job_by_id(job_id.into_inner()) {
        Ok(job) => HttpResponse::Ok().json(job),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/jobs")]
async fn create_job(db: Data<Database>, job: web::Json<JobReq>) -> impl Responder {
    match db.insert_job(job.into_inner()) {
        Ok(job) => HttpResponse::Ok().json(job),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/companies")]
async fn get_companies(db: Data<Database>) -> impl Responder {
    match db.get_companies() {
        Ok(companies) => HttpResponse::Ok().json(companies),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/companies")]
async fn create_company(db: Data<Database>, company: web::Json<CompanyReq>) -> impl Responder {
    match db.insert_company(company.into_inner()) {
        Ok(company) => HttpResponse::Ok().json(company),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
