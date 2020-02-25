-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  active BOOLEAN NOT NULL DEFAULT 'f',
  sign_in_count bigint NOT NULL
);
