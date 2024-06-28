use std::sync::Mutex;
use actix_web::{get, HttpResponse, post, Responder, Scope, web};
use actix_web::web::Data;
use anyhow::Error;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::db::Database;
use crate::models::{Company, Job};

pub fn api_routes() -> Scope {
    web::scope("")
        .service(hello)
        .service(echo)
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// #[post("/posts")]
// async fn create_post(db: Data<Database>, post: web::Json<Post>) -> impl Responder {
//     println!("Post {:?}", post);
//     match db.create_post(post.into_inner()) {
//         Ok(post) => HttpResponse::Ok().json(post),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }

#[get("/jobs")]
async fn get_jobs(db: Data<Database>) -> impl Responder {
    match db.get_jobs_and_companies() {
        Ok(jobs) => HttpResponse::Ok().json(jobs),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[get("/jobs/{id}")]
async fn get_job_by_id(db: Data<Database>, job_id: web::Path<i32>) -> impl Responder {
    match db.find_job_by_id(job_id.into_inner()) {
        Ok(job) => HttpResponse::Ok().json(job),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[post("/jobs")]
async fn create_job(db: Data<Database>, job: web::Json<Job>) -> impl Responder {
    let job = match db.insert_job(job.into_inner()) {
        Ok(job) => job,
        Err(_) => { return HttpResponse::InternalServerError().finish(); }
    };

    match db.find_job_by_id(job.id) {
        Ok(job_company) => HttpResponse::Ok().json(job_company),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

// fn merge_jobs_and_companies_info((job, company): (Job, Company)) -> JobWithCompany {
//     JobWithCompany {
//         id: job.id,
//         date_posted: job.date_posted,
//         title: job.title.clone(),
//         description: job.clone().description.unwrap_or("".into()),
//         company_id: job.company,
//         company_name: company.name,
//         company_description: company.description.unwrap_or("".into()),
//         company_logo: company.logo_cid.unwrap_or("".into()),
//         pay: job.pay.unwrap_or("".into()),
//         job_location: job.job_location,
//         remote: job.remote,
//         job_type: job.job_type,
//     }
// }



// #[derive(Serialize, Deserialize)]
// pub struct Job {
//     pub id: i32,
//     pub date_posted: NaiveDate,
//     pub title: String,
//     pub description: Option<String>,
//     pub company: i32,
//     pub pay: Option<String>,
//     pub job_location: String,
//     pub remote: bool,
//     pub job_type: String
// }