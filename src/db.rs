use crate::models::{Job, JobRes, JobReq, Company, CompanyReq};
use crate::schema::companies::dsl::companies;
use anyhow::{Context, Error};
use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use crate::utils;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Database {
    pool: DBPool,
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

    pub fn get_jobs(&self) -> Result<Vec<JobRes>, Error> {
        use crate::schema::jobs::dsl::*;

        Ok(jobs
            .inner_join(companies)
            .load::<(Job, Company)>(&mut self.pool.get()?)?
            .into_iter()
            .map(|(_job, _company): (Job, Company)| JobRes::build_from(_job, _company))
            .collect())
    }

    pub fn insert_job(&self, job: JobReq) -> Result<JobRes, Error> {
        use crate::schema::companies::dsl::*;
        use crate::schema::jobs::dsl::*;

        let conn = &mut self.pool.get()?;
        let _company = companies
            .find(&job.company_id)
            .first::<Company>(conn)
            .context("Company linked to job not found")?;

        let _job = Job {
            id: utils::generate_url_id(&job.title),
            date_posted: None,
            title: job.title,
            description: job.description,
            company_id: job.company_id,
            pay: job.pay,
            location: job.location,
            remote: job.remote,
            job_type: job.job_type,
        };

        Ok(_job
            .insert_into(jobs)
            .get_result(conn)
            .map(|j| JobRes::build_from(j, _company))?)
    }

    pub fn find_job_by_id(&self, id: String) -> Result<JobRes, Error> {
        use crate::schema::companies::dsl::companies;
        use crate::schema::jobs::dsl::{id as jobs_id, jobs};

        Ok(jobs
            .inner_join(companies)
            .filter(jobs_id.eq(id))
            .first::<(Job, Company)>(&mut self.pool.get()?)
            .map(|(_job, _company): (Job, Company)| JobRes::build_from(_job, _company))?)
    }

    pub fn insert_company(&self, _company: CompanyReq) -> Result<Company, Error> {
        use crate::schema::companies::dsl::*;

        let _company = Company {
            id: utils::generate_url_id(&_company.name),
            date_added: None,
            name: _company.name,
            website: _company.website,
            logo_cid: _company.logo_cid,
            description: _company.description,
        };

        Ok(_company
            .insert_into(companies)
            .get_result(&mut self.pool.get()?)?)
    }

    pub fn get_companies(&self) -> Result<Vec<Company>, Error> {
        use crate::schema::companies::dsl::*;

        Ok(companies.load(&mut self.pool.get()?)?)
    }

    pub fn _get_skills(&self, _job_id: i32) -> Result<Vec<String>, Error> {
        todo!()
    }
}
