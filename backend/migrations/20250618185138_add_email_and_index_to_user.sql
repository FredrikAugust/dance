-- Add migration script here
CREATE UNIQUE INDEX users_email_index ON users (email);
