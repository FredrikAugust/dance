-- Add migration script here
CREATE TABLE companies (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    location TEXT NOT NULL
);

CREATE TABLE opportunities (
    id UUID PRIMARY KEY,
    company_id UUID NOT NULL REFERENCES companies(id),
    location TEXT NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP,
    image_urls TEXT[] NOT NULL,
    description TEXT NOT NULL,
    application_url TEXT
);