-- Add up migration script here
CREATE TABLE urls (
  id VARCHAR(6) PRIMARY KEY,
  url VARCHAR NOT NULL
);