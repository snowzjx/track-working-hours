-- Your SQL goes here
CREATE TABLE projects (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  info TEXT NOT NULL,
  priority INTEGER NOT NULL
)