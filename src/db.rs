use std::env;
use diesel::prelude::*;
use dotenvy::dotenv;
use anyhow::Error;
use diesel::associations::HasTable;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager};
use crate::models::{Company, Job, JobWithCompany};
use crate::schema::companies::dsl::companies;
use crate::schema::jobs::dsl::jobs;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;
// pub type DieselResult<T> = Result<T, diesel::result::Error>;

#[derive(Clone)]
pub struct Database {
    pool: DBPool
}

impl Database {
    /// Creates a new DB connection pool.
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn get_jobs_and_companies(&self) -> Result<Vec<JobWithCompany>, Error> {
        Ok(jobs
            .inner_join(companies)
            .load::<(Job, Company)>(&mut self.pool.get()?)?
            .into_iter()
            .map(|(job, company)| JobWithCompany { job, company })
            .collect()
        )
    }

    pub fn get_jobs(&self) -> Result<Vec<Job>, Error> {
        Ok(jobs.load(&mut self.pool.get()?)?)
    }

    pub fn insert_job(&self, job: Job) -> Result<Job, Error> {
        Ok(job
            .insert_into(jobs)
            .get_result(&mut self.pool.get()?)?
        )
    }

    pub fn find_job_by_id(&self, id: i32) -> Result<JobWithCompany, Error> {
        todo!()
        // Ok(jobs
        //     .inner_join(companies)
        //     // .load::<(Job, Company)>(&mut self.pool.get()?)?
        //     .find(id)
        //     .into_iter()
        //     .map(|(job, company)| JobWithCompany { job, company })
        //     .first(&mut self.pool.get()?)?
        // )
    }

    pub fn get_skills(&self, job_id: i32) -> Result<Vec<String>, Error> {
        todo!()
    }
}
