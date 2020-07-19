-- Your SQL goes here
CREATE TABLE trackings (
  username VARCHAR NOT NULL ,
  project_id INTEGER NOT NULL,
  created_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  recorded_time REAL NOT NULL,
  PRIMARY KEY(username, project_id, created_time),
  CONSTRAINT fk_username
      FOREIGN KEY(username) 
	    REFERENCES users(username),
  CONSTRAINT fk_project_id
   FOREIGN KEY(project_id) 
      REFERENCES projects(id)
)