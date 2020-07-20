use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use rocket::request::Form;
use rocket::request::FlashMessage;

use std::collections::HashMap;

use crate::requests::login_form::LoginForm;
use crate::requests::track_form::TrackForm;

use data::models::*;
use data::*;

pub struct UserWrapper {
    pub user: Option<User>,
}

impl Default for UserWrapper { 
    fn default() -> Self { 
        UserWrapper {
            user: None
        }
    }
}

pub type Session<'a> = rocket_session::Session<'a, UserWrapper>;

#[get("/")]
pub fn index(session: Session) -> std::result::Result<Template, Redirect> {
    session.tap(|user_wrapper| {
        match &user_wrapper.user {
            Some(user) => {
                let mut context = Context::new();
                let display_name = &user.display_name;
                context.insert("display_name", display_name);
                let assigned_projects = select_assigned_projects(user);
                match assigned_projects {
                    Ok(assigned_projects) => context.insert("assigned_projects", &assigned_projects),
                    Err(_) => (),
                }
                Ok(Template::render("index", context))
            }
            None => {
                Err(Redirect::to(uri!(super::pages::login)))
            }
        }
    })
}

#[get("/login")]
pub fn login() -> Template {
    let context = HashMap::<&str, &str>::new();
    Template::render("login", context)
}

#[post("/login", data = "<form>")]
pub fn post_login(session: Session, form: Form<LoginForm>) -> std::result::Result<Redirect, Flash<Redirect>> {
    let inner_form = form.into_inner();
    let username = inner_form.username;
    let password = inner_form.password;

    let user = select_user(&username, &password);

    match user {
        Ok(Option::Some(user)) => {
            session.tap(|user_wrapper| {
                user_wrapper.user = Some(user);
            });
            Ok(Redirect::to(uri!(super::pages::index)))
        },
        _ => Err(Flash::error(Redirect::to("/error"), "No user found ..."))
    }
}

#[get("/logout")]
pub fn logout(session: Session) -> std::result::Result<Redirect, Flash<Redirect>> {
    session.tap(|user_wrapper| {
        user_wrapper.user = None;
        Ok(Redirect::to(uri!(super::pages::login))) 
    })
}

#[get("/track")]
pub fn tracking(session: Session) -> std::result::Result<Template, Redirect> {
    session.tap(|user_wrapper| {
        match &user_wrapper.user {
            Some(user) => {
                let mut context = Context::new();
                let project_trackings = select_project_trackings_by_user(user);
                match project_trackings {
                    Ok(project_trackings) => {
                        context.insert("project_trackings", &project_trackings)
                    },
                    Err(_) => ()
                }
                Ok(Template::render("track", context))
            }
            None => {
                Err(Redirect::to(uri!(super::pages::login)))
            }
        }
    })
}

#[post("/track", data = "<form>")]
pub fn post_tracking(session: Session, form: Form<TrackForm>) -> std::result::Result<Redirect, Flash<Redirect>> {
    session.tap(|user_wrapper| {
        match &user_wrapper.user {
            Some(user) => {
                let map = &form.map;
                let mut new_trackings = Vec::<(&str, i32, f32)>::new();
                for (project_id, recorded_time) in map {
                    match (project_id.parse::<i32>(), recorded_time.parse::<f32>()) {
                        (Ok(project_id), Ok(recorded_time)) => {
                            new_trackings.push((&user.username, project_id, recorded_time));
                        }
                        _ => return Err(Flash::error(Redirect::to("/error"), "Error phasing values ..."))
                    }
                }
                match create_trackings(new_trackings) {
                    Ok(_) => Ok(Redirect::to(uri!(super::pages::index))),
                    Err(err) => {
                        // let err_msg = err.description();
                        Err(Flash::error(Redirect::to("/error"), format!("Error storing trackings ...\nCaused by:\n\t{:?}", err)))
                    }
                }
            }
            None => {
                Err(Flash::error(Redirect::to("/error"), "No user found ..."))
            }
        }
    })
}

#[get("/error")]
pub fn error(flash: Option<FlashMessage>) -> String {
    flash.map(|msg| format!("{}: {}", msg.name(), msg.msg()))
         .unwrap_or_else(|| "Error!".to_string())
}