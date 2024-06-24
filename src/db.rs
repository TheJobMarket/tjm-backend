use std::env;
use diesel::{Connection, PgConnection, r2d2, RunQueryDsl};
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use anyhow::Error;
use crate::models::Post;
use crate::schema::posts::dsl::posts;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

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

    pub fn create_post(&self, post: Post) -> Result<Post, Error> {
        diesel::insert_into(posts)
            .values(&post)
            .execute(&mut self.pool.get().unwrap())?;
        Ok(post)
    }
}

// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();
//
//     let database_url = env::var("DATABASE_URL").expect("Database URL must be set.");
//     PgConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }
//
