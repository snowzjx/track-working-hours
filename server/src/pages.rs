use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use rocket::request::Form;
use rocket::request::FlashMessage;

use std::collections::HashMap;

use chrono::NaiveDate;

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

#[get("/forget/<year>/<month>/<day>")]
pub fn forget(session: Session, year:i32, month:u32, day:u32) -> std::result::Result<Template, Flash<Redirect>> {
    session.tap(|user_wrapper| {
        match &user_wrapper.user {
            Some(user) => {
                let mut context = Context::new();
                let display_name = &user.display_name;
                context.insert("display_name", display_name);
                match NaiveDate::from_ymd_opt(year, month, day) {
                    Some(_) => {
                        context.insert("year", &year);
                        context.insert("month", &month);
                        context.insert("day", &day);
                    },
                    None => {
                        return Err(Flash::error(Redirect::to("/error"), "You should be careful since you already forget something ..."))
                    }
                }
                let assigned_projects = select_assigned_projects(user);
                match assigned_projects {
                    Ok(assigned_projects) => context.insert("assigned_projects", &assigned_projects),
                    Err(_) => (),
                }
                Ok(Template::render("forget", context))
            }
            None => {
                Err(Flash::error(Redirect::to("/error"), "No user found ..."))
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
                            if recorded_time == 0.0 {
                                continue;
                            }
                            new_trackings.push((&user.username, project_id, recorded_time));
                        }
                        _ => return Err(Flash::error(Redirect::to("/error"), "Error phasing values ..."))
                    }
                }
                if new_trackings.is_empty() {
                    return Err(Flash::error(Redirect::to("/error"), "Please fill in some values ..."))
                }
                match create_trackings(new_trackings) {
                    Ok(_) => Ok(Redirect::to(uri!(super::pages::tracking))),
                    Err(err) => {
                        // let err_msg = err.description();
                        Err(Flash::error(Redirect::to("/error"), format!("Error insert trackings ...\nCaused by:\n\t{:?}", err)))
                    }
                }
            }
            None => {
                Err(Flash::error(Redirect::to("/error"), "No user found ..."))
            }
        }
    })
}

#[post("/forget/<year>/<month>/<day>", data = "<form>")]
pub fn post_forget(session: Session, form: Form<TrackForm>, year:i32, month:u32, day:u32) -> std::result::Result<Redirect, Flash<Redirect>> {
    session.tap(|user_wrapper| {
        match &user_wrapper.user {
            Some(user) => {
                let date = match NaiveDate::from_ymd_opt(year, month, day) {
                    Some(date) => date,
                    None => {
                        return Err(Flash::error(Redirect::to("/error"), "Wrong date ..."))
                    }
                };
                let map = &form.map;
                let mut new_trackings = Vec::<(&str, i32, f32, NaiveDate)>::new();
                for (project_id, recorded_time) in map {
                    match (project_id.parse::<i32>(), recorded_time.parse::<f32>()) {
                        (Ok(project_id), Ok(recorded_time)) => {
                            if recorded_time == 0.0 {
                                continue;
                            }
                            new_trackings.push((&user.username, project_id, recorded_time, date));
                        }
                        _ => return Err(Flash::error(Redirect::to("/error"), "Error phasing values ..."))
                    }
                }
                if new_trackings.is_empty() {
                    return Err(Flash::error(Redirect::to("/error"), "Please fill in some values ..."))
                }
                match create_trackings_with_date(new_trackings) {
                    Ok(_) => Ok(Redirect::to(uri!(super::pages::tracking))),
                    Err(err) => {
                        // let err_msg = err.description();
                        Err(Flash::error(Redirect::to("/error"), format!("Error insert trackings ...\nCaused by:\n\t{:?}", err)))
                    }
                }
            }
            None => {
                Err(Flash::error(Redirect::to("/error"), "No user found ..."))
            }
        }
    })
}

#[get("/track/del/<track_id>")]
pub fn del_tracking(session: Session, track_id: i32) -> std::result::Result<Redirect, Flash<Redirect>> {
    session.tap(|user_wrapper| {
        match &user_wrapper.user {
            Some(user) => {
                let username = &user.username;
                match del_tracking_by_id(username, track_id) {
                    Ok(_) => Ok(Redirect::to(uri!(super::pages::tracking))),
                    Err(err) => {
                        // let err_msg = err.description();
                        Err(Flash::error(Redirect::to("/error"), format!("Error delete trackings ...\nCaused by:\n\t{:?}", err)))
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