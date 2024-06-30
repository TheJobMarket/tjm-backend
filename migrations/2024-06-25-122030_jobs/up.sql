-- Your SQL goes here
CREATE TABLE jobs (
    id SERIAL PRIMARY KEY,
    date_posted DATE DEFAULT CURRENT_DATE NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    company INT REFERENCES companies(id) NOT NULL,
    pay VARCHAR,
    job_location TEXT NOT NULL,
    remote BOOLEAN NOT NULL,
    job_type VARCHAR NOT NULL
)