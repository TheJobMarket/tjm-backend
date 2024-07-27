use chrono::NaiveDate;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = crate::schema::jobs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Job {
    pub id: i32,
    pub date_posted: NaiveDate,
    pub title: String,
    pub description: Option<String>,
    pub company: i32,
    pub pay: Option<String>,
    pub job_location: String,
    pub remote: bool,
    pub job_type: String,
}

#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = crate::schema::jobs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct JobReq {
    pub title: String,
    pub description: Option<String>,
    pub company: i32,
    pub pay: Option<String>,
    pub job_location: String,
    pub remote: bool,
    pub job_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobRes {
    pub id: i32,
    pub date_posted: NaiveDate,
    pub title: String,
    pub description: Option<String>,
    pub company: Company,
    pub pay: Option<String>,
    pub job_location: String,
    pub remote: bool,
    pub job_type: String,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = crate::schema::companies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub website: Option<String>,
    pub logo_cid: Option<String>,
    pub description: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = crate::schema::companies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CompanyReq {
    pub name: String,
    pub website: Option<String>,
    pub logo_cid: Option<String>,
    pub description: Option<String>,
}

impl JobRes {
    pub fn build_from(job: Job, company: Company) -> JobRes {
        JobRes {
            id: job.id,
            date_posted: job.date_posted,
            title: job.title,
            description: job.description,
            company,
            pay: job.pay,
            job_location: job.job_location,
            remote: job.remote,
            job_type: job.job_type,
        }
    }
}
