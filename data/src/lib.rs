#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use chrono::NaiveDate;

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

use self::models::{Tracking, NewTracking, NewTrackingWithDate};

pub fn create_tracking<'a>(_username: &'a str, _project_id: i32, _recorded_time: f32) -> Result<Tracking, diesel::result::Error> {
    use schema::trackings;

    let conn = establish_connection();

    let new_tracking = NewTracking {
        username: _username,
        project_id: _project_id,
        recorded_time: _recorded_time,
    };
    
    diesel::insert_into(trackings::table)
        .values(&new_tracking)
        .get_result(&conn)
}

pub fn create_trackings<'a>(_new_trackings: Vec<(&'a str, i32, f32)>) -> Result<Vec<Tracking>, diesel::result::Error> {
    use schema::trackings;

    let conn = establish_connection();

    let new_trackings = _new_trackings.iter().map(|value| {
        NewTracking {
            username: value.0,
            project_id: value.1,
            recorded_time: value.2,
        }
    }).collect::<Vec<NewTracking>>();
    
    diesel::insert_into(trackings::table)
        .values(new_trackings)
        .get_results(&conn)
}

pub fn create_trackings_with_date<'a>(_new_trackings: Vec<(&'a str, i32, f32, NaiveDate)>) -> Result<Vec<Tracking>, diesel::result::Error> {
    use schema::trackings;

    let conn = establish_connection();

    let new_trackings = _new_trackings.iter().map(|value| {
        NewTrackingWithDate {
            username: value.0,
            project_id: value.1,
            created_time: value.3,
            recorded_time: value.2,
        }
    }).collect::<Vec<NewTrackingWithDate>>();
    
    diesel::insert_into(trackings::table)
        .values(new_trackings)
        .get_results(&conn)
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

pub fn select_project_trackings_by_user<'a>(_user: &'a User) -> Result<Vec<(Project, Tracking)>, diesel::result::Error> {
    use schema::projects;
    use schema::trackings;

    let conn = establish_connection();

    Tracking::belonging_to(_user)
        .inner_join(projects::table)
        .order(trackings::created_time.desc())
        .select((projects::all_columns, trackings::all_columns))
        .load::<(Project, Tracking)>(&conn)
}

pub fn del_tracking_by_id<'a>(_username: &'a str, _track_id: i32) -> Result<usize, diesel::result::Error> {
    use schema::trackings::dsl::*;

    let conn = establish_connection();

    diesel::delete(trackings.filter(id.eq(_track_id)).filter(username.eq(_username)))
        .execute(&conn)

}

use self::models::{Assign, NewAssign};

pub fn create_assign<'a>(_username: &'a str, _project_id: i32) -> Result<Assign, diesel::result::Error> {
    use schema::assigns;

    let conn = establish_connection();

    let new_assign = NewAssign {
        username: _username,
        project_id: _project_id,
    };
    
    diesel::insert_into(assigns::table)
        .values(&new_assign)
        .get_result(&conn)
}


pub fn select_assigned_projects<'a>(_user: &'a User) -> Result<Vec<Project>, diesel::result::Error> {
    use schema::projects;

    let conn = establish_connection();

    Assign::belonging_to(_user)
        .inner_join(projects::table)
        .select(projects::all_columns)
        .load::<Project>(&conn)
}