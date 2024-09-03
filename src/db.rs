use crate::models::{Job, JobRes, JobReq, Company, CompanyReq};
use crate::schema::companies::dsl::companies;
use anyhow::{Context, Error};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Credentials;
use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use crate::utils;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub s3: S3Client,
}

impl AppState {
    pub async fn init() -> Self {
        AppState {
            db: Database::new().await,
            s3: S3Client::new().await,
        }
    }
}

#[derive(Clone)]
pub struct S3Client(Client);

impl S3Client {
    pub async fn new() -> Self {
        let access_key_id = std::env::var("AWS_ACCESS_KEY_ID")
            .expect("AWS_ACCESS_KEY_ID not set in .env");
        let secret_access_key = std::env::var("AWS_SECRET_ACCESS_KEY")
            .expect("AWS_SECRET_ACCESS_KEY not set in .env");
        let aws_url = std::env::var("AWS_URL")
            .expect("AWS_URL not set in .env");

        // We pass `None` instead of a session token.
        let creds = Credentials::new(
            access_key_id,
            secret_access_key,
            None,
            None,
            "tjm-server"
        );

        let cfg = aws_config::defaults(BehaviorVersion::latest())
            .endpoint_url(aws_url)
            .region(Region::new("eu-west-3"))
            .credentials_provider(creds)
            .load().await;

        S3Client(Client::new(&cfg))
    }
}

#[derive(Clone)]
pub struct Database(DBPool);

impl Database {
    /// Creates a new DB connection pool.
    pub async fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        Database(pool)
    }

    pub fn get_jobs(&self) -> Result<Vec<JobRes>, Error> {
        use crate::schema::jobs::dsl::*;

        Ok(jobs
            .inner_join(companies)
            .load::<(Job, Company)>(&mut self.0.get()?)?
            .into_iter()
            .map(|(_job, _company): (Job, Company)| _job.to_res(_company))
            .collect())
    }

    pub fn insert_job(&self, job_req: JobReq) -> Result<JobRes, Error> {
        use crate::schema::companies::dsl::*;
        use crate::schema::jobs::dsl::*;

        let conn = &mut self.0.get()?;
        let _company = companies
            .find(&job_req.company_id)
            .first::<Company>(conn)
            .context("Company linked to job not found")?;

        let _job = Job::from_req(job_req);

        Ok(_job
            .insert_into(jobs)
            .get_result(conn)
            .map(|j: Job| j.to_res(_company))?)
    }

    pub fn find_job_by_id(&self, id: String) -> Result<JobRes, Error> {
        use crate::schema::companies::dsl::companies;
        use crate::schema::jobs::dsl::{id as jobs_id, jobs};

        Ok(jobs
            .inner_join(companies)
            .filter(jobs_id.eq(id))
            .first::<(Job, Company)>(&mut self.0.get()?)
            .map(|(_job, _company): (Job, Company)| _job.to_res(_company))?)
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
            .get_result(&mut self.0.get()?)?)
    }

    pub fn get_companies(&self) -> Result<Vec<Company>, Error> {
        use crate::schema::companies::dsl::*;

        Ok(companies.load(&mut self.0.get()?)?)
    }

    pub fn _get_skills(&self, _job_id: i32) -> Result<Vec<String>, Error> {
        todo!()
    }
}
