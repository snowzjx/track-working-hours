-- Your SQL goes here
CREATE TABLE assigns (
  username VARCHAR NOT NULL ,
  project_id INTEGER NOT NULL,
  PRIMARY KEY(username, project_id),
  CONSTRAINT fk_username
      FOREIGN KEY(username) 
	    REFERENCES users(username),
  CONSTRAINT fk_project_id
   FOREIGN KEY(project_id) 
      REFERENCES projects(id)
)