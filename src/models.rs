use chrono::NaiveDate;
use diesel::{Insertable, Queryable, Selectable};
use diesel::sql_types::Date;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
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
    pub job_type: String
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::companies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub website: Option<String>,
    pub logo_cid: Option<String>,
    pub description: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JobWithCompany {
    pub job: Job,
    pub company: Company
}

// #[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
// pub struct JobFull {
//     pub id: i32,
//     pub date_posted: NaiveDate,
//     pub title: String,
//     pub description: String,
//     pub company_id: i32,
//     pub company_name: String,
//     pub company_description: String,
//     pub company_logo: String,
//     pub pay: String,
//     pub job_location: String,
//     pub remote: bool,
//     pub job_type: String,
// }

#[derive(Serialize, Deserialize)]
pub struct Id {
    pub id: i32
}

