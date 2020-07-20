#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Project, NewProject};

pub fn show_project() -> Result<Vec<Project>, diesel::result::Error> {
    use schema::projects::dsl::*;

    let conn = establish_connection();

    projects.order(priority.desc())
        .load::<Project>(&conn)
}

pub fn create_project<'a>(_name: &'a str, _info: &'a str, _priority: i32) -> Result<Project, diesel::result::Error> {
    use schema::projects;

    let conn = establish_connection();

    let new_project = NewProject {
        name: _name,
        info: _info,
        priority: _priority,
    };

    diesel::insert_into(projects::table)
        .values(&new_project)
        .get_result(&conn)
}

use self::models::{User, NewUser};

pub fn select_user<'a>(_username: &'a str, _password: &'a str) -> Result<Option<User>, diesel::result::Error> {
    use schema::users::dsl::*;

    let conn = establish_connection();

    users.select((username, display_name))
        .filter(username.eq(_username))
        .filter(password.eq(_password))
        .first::<User>(&conn)
        .optional()
}

pub fn create_user<'a>(_username: &'a str, _password: &'a str, _display_name: &'a str) -> Result<User, diesel::result::Error> {
    use schema::users;

    let conn = establish_connection();
    
    let new_user = NewUser {
        username: _username,
        password: _password,
        display_name: _display_name,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning((users::username, users::display_name))
        .get_result(&conn)
}

use self::models::{Tracking, NewTracking};

pub fn create_tracking<'a>(_ussername: &'a str, _project_id: i32, _recorded_time: f32) -> Result<Tracking, diesel::result::Error> {
    use schema::trackings;

    let conn = establish_connection();

    let new_tracking = NewTracking {
        username: _ussername,
        project_id: _project_id,
        recorded_time: _recorded_time,
    };
    
    diesel::insert_into(trackings::table)
        .values(&new_tracking)
        .get_result(&conn)
}

pub fn select_trackings_by_user<'a>(_user: &'a User) -> Result<Vec<Tracking>, diesel::result::Error> {
    use schema::trackings::dsl::*;

    let conn = establish_connection();

    Tracking::belonging_to(_user)
        .order(created_time.desc())
        .load::<Tracking>(&conn)
}

pub fn select_trackings_by_project<'a>(_project: &'a Project) -> Result<Vec<Tracking>, diesel::result::Error> {
    use schema::trackings::dsl::*;

    let conn = establish_connection();

    Tracking::belonging_to(_project)
        .order(created_time.desc())
        .load::<Tracking>(&conn)
}

pub fn select_project_trackings_by_user<'a>(_user: &'a User) -> Result<Vec<(String, i32, String)>, diesel::result::Error> {
    use schema::trackings;
    use schema::projects;

    let conn = establish_connection();

    Tracking::belonging_to(_user)
        .inner_join(projects::table)
        .select((trackings::username, projects::id, projects::name))
        .load(&conn)
}