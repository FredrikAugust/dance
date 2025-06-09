-- Add migration script here
ALTER TABLE opportunities
ADD COLUMN title TEXT NOT NULL;

ALTER TABLE opportunities
ADD COLUMN application_deadline TIMESTAMP WITH TIME ZONE;