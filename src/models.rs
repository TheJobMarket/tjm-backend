use chrono::NaiveDate;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

// TODO: just make different types for request, tuple, and response and then merge any if possible.

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::jobs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Job {
    pub id: String,
    pub date_posted: Option<NaiveDate>,
    pub title: String,
    pub description: Option<String>,
    pub company_id: String,
    pub pay: Option<String>,
    pub location: String,
    pub remote: bool,
    pub job_type: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobReq {
    pub title: String,
    pub description: Option<String>,
    pub company_id: String,
    pub pay: Option<String>,
    pub location: String,
    pub remote: bool,
    pub job_type: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobRes {
    pub id: String,
    pub date_posted: NaiveDate,
    pub title: String,
    pub description: Option<String>,
    pub company: Company,
    pub pay: Option<String>,
    pub location: String,
    pub remote: bool,
    pub job_type: String,
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
    pub fn build_from(job: Job, company: Company) -> JobRes {
        JobRes {
            id: job.id,
            date_posted: job.date_posted.expect("There should be a default value"),
            title: job.title,
            description: job.description,
            company,
            pay: job.pay,
            location: job.location,
            remote: job.remote,
            job_type: job.job_type,
        }
    }
}
