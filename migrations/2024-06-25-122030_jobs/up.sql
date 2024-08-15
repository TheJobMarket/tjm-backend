-- Your SQL goes here
CREATE TABLE jobs (
    id VARCHAR PRIMARY KEY,
    date_posted DATE DEFAULT CURRENT_DATE,
    title VARCHAR NOT NULL,
    description TEXT,
    company_id VARCHAR REFERENCES companies(id) NOT NULL,
    active BOOLEAN NOT NULL,
    application_url VARCHAR,
    pay_min INTEGER,
    pay_max INTEGER,
    city VARCHAR,
    country VARCHAR,
    languages VARCHAR,
    workplace VARCHAR,
    job_type VARCHAR
)