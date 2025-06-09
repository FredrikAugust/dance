-- Add migration script here
CREATE TABLE companies (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    website_url TEXT
);

ALTER TABLE opportunities
ADD COLUMN company_id UUID NOT NULL REFERENCES companies(id);