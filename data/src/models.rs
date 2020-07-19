use serde::Serialize;

use super::schema::projects;
#[derive(Identifiable, Queryable, Serialize, Debug)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub info: String,
    pub priority: i32,
}

#[derive(Insertable)]
#[table_name="projects"]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub info: &'a str,
    pub priority: i32,
}

use super::schema::users;
#[derive(Identifiable, Queryable, Serialize, Debug)]
#[primary_key(username)]
pub struct User {
    pub username: String,
    pub display_name: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub display_name: &'a str,
}

use super::schema::trackings;
#[derive(Identifiable, Queryable, Associations, Serialize, Debug)]
#[primary_key(username, project_id, created_time)]
#[belongs_to(User, foreign_key = "username")]
#[belongs_to(Project)]
pub struct Tracking {
    pub username: String,
    pub project_id: i32,
    pub created_time: chrono::NaiveDateTime,
    pub recorded_time: f32,
}

#[derive(Insertable)]
#[table_name="trackings"]
pub struct NewTracking<'a> {
    pub username: &'a str,
    pub project_id: i32,
    pub recorded_time: f32,
}