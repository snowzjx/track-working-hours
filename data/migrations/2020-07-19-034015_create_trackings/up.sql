-- Your SQL goes here
CREATE TABLE trackings (
  id SERIAL PRIMARY KEY, 
  username VARCHAR NOT NULL ,
  project_id INTEGER NOT NULL,
  created_time DATE NOT NULL DEFAULT CURRENT_DATE,
  recorded_time REAL NOT NULL,
  UNIQUE(username, project_id, created_time),
  CONSTRAINT fk_username
      FOREIGN KEY(username) 
	    REFERENCES users(username),
  CONSTRAINT fk_project_id
   FOREIGN KEY(project_id) 
      REFERENCES projects(id)
)