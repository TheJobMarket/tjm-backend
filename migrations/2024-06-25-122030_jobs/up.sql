-- Your SQL goes here
CREATE TABLE jobs (
    id VARCHAR PRIMARY KEY,
    date_posted DATE DEFAULT CURRENT_DATE,
    title VARCHAR NOT NULL,
    description TEXT,
    company_id VARCHAR REFERENCES companies(id) NOT NULL,
    pay VARCHAR,
    location TEXT NOT NULL,
    remote BOOLEAN NOT NULL,
    job_type VARCHAR NOT NULL
)