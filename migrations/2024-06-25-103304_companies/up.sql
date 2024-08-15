-- Your SQL goes here
CREATE TABLE companies (
    id VARCHAR PRIMARY KEY,
    date_added DATE DEFAULT CURRENT_DATE,
    name VARCHAR NOT NULL,
    website VARCHAR,
    logo_cid VARCHAR,
    description TEXT
)