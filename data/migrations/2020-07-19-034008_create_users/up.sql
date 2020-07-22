-- Your SQL goes here
CREATE TABLE users (
  username VARCHAR NOT NULL PRIMARY KEY,
  password VARCHAR NOT NULL,
  display_name VARCHAR NOT NULL,
  is_admin BOOLEAN NOT NULL DEFAULT FALSE
)