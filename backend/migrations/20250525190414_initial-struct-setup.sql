-- Add migration script here
CREATE TABLE opportunities (
    id UUID PRIMARY KEY,
    location TEXT NOT NULL,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE,
    image_urls TEXT[] NOT NULL,
    description TEXT NOT NULL,
    application_url TEXT
);
