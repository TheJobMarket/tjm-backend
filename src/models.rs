use chrono::NaiveDate;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use crate::utils;

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::jobs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Job {
    pub id: String,
    pub date_posted: Option<NaiveDate>,
    pub title: String,
    pub description: Option<String>,
    pub company_id: String,
    pub active: bool,
    pub application_url: Option<String>,
    pub pay_min: Option<i32>,
    pub pay_max: Option<i32>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub languages: Option<String>,
    pub workplace: Option<String>,
    pub job_type: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobReq {
    pub title: String,
    pub description: Option<String>,
    pub company_id: String,
    pub active: bool,
    pub application_url: Option<String>,
    pub pay_min: Option<i32>,
    pub pay_max: Option<i32>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub languages: Option<String>,
    pub workplace: Option<String>,
    pub job_type: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobRes {
    pub id: String,
    pub date_posted: NaiveDate,
    pub title: String,
    pub description: Option<String>,
    pub company: Company,
    pub active: bool,
    pub application_url: Option<String>,
    pub pay_min: Option<i32>,
    pub pay_max: Option<i32>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub languages: Option<String>,
    pub workplace: Option<String>,
    pub job_type: Option<String>,
}

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::companies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Company {
    pub id: String,
    pub date_added: Option<NaiveDate>,
    pub name: String,
    pub website: Option<String>,
    pub logo_cid: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyReq {
    pub name: String,
    pub website: Option<String>,
    pub logo_cid: Option<String>,
    pub description: Option<String>,
}

impl JobRes {
    pub fn from_job(job: Job, company: Company) -> Self {
        JobRes {
            id: job.id,
            date_posted: job.date_posted.expect("There should be a default value"),
            title: job.title,
            description: job.description,
            company,
            active: job.active,
            application_url: job.application_url,
            pay_min: job.pay_min,
            pay_max: job.pay_max,
            city: job.city,
            country: job.country,
            languages: job.languages,
            workplace: job.workplace,
            job_type: job.job_type,
        }
    }
}

impl Job {
    pub fn from_req(job_req: JobReq) -> Self {
        Job {
            id: utils::generate_url_id(&job_req.title),
            date_posted: None,
            title: job_req.title,
            description: job_req.description,
            company_id: job_req.company_id,
            active: job_req.active,
            application_url: job_req.application_url,
            pay_min: job_req.pay_min,
            pay_max: job_req.pay_max,
            city: job_req.city,
            country: job_req.country,
            languages: job_req.languages,
            workplace: job_req.workplace,
            job_type: job_req.job_type,
        }
    }
}
