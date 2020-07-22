use serde::{Serialize, Deserialize};

use super::schema::projects;
#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub status: String, 
    pub info: String,
    pub priority: i32,
}

impl std::hash::Hash for Project {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write_i32(self.id);
        state.finish();
    }
}

impl PartialEq for Project {
    fn eq(&self, other: &Project) -> bool {
        self.id == other.id
    }
}
impl Eq for Project {}

#[derive(Insertable)]
#[table_name="projects"]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub status: &'a str,
    pub info: &'a str,
    pub priority: i32,
}

use super::schema::users;
#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[primary_key(username)]
pub struct User {
    pub username: String,
    pub display_name: String,
    pub is_admin: bool,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub display_name: &'a str,
    pub is_admin: bool,
}

use super::schema::trackings;
#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Debug)]
#[belongs_to(User, foreign_key = "username")]
#[belongs_to(Project)]
pub struct Tracking {
    pub id: i32,
    pub username: String,
    pub project_id: i32,
    pub created_time: chrono::NaiveDate,
    pub recorded_time: f32,
}

#[derive(Insertable)]
#[table_name="trackings"]
pub struct NewTracking<'a> {
    pub username: &'a str,
    pub project_id: i32,
    pub recorded_time: f32,
}

#[derive(Insertable)]
#[table_name="trackings"]
pub struct NewTrackingWithDate<'a> {
    pub username: &'a str,
    pub project_id: i32,
    pub created_time: chrono::NaiveDate,
    pub recorded_time: f32,
}

use super::schema::assigns;
#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Debug)]
#[primary_key(username, project_id)]
#[belongs_to(User, foreign_key = "username")]
#[belongs_to(Project)]
pub struct Assign {
    pub username: String,
    pub project_id: i32,
}

#[derive(Insertable)]
#[table_name="assigns"]
pub struct NewAssign<'a> {
    pub username: &'a str,
    pub project_id: i32,
}