use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use rocket::request::Form;

use std::collections::HashMap;

use crate::requests::login_form::LoginForm;

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
        },
        _ => (),
    }

    Ok(Redirect::to(uri!(super::pages::index)))
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
                let trackings = select_trackings_by_user(user);
                match trackings {
                    Ok(trackings) => context.insert("trackings", &trackings),
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