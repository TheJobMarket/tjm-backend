-- Your SQL goes here
CREATE TABLE companies (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    website VARCHAR,
    logo_cid VARCHAR,
    description TEXT
)