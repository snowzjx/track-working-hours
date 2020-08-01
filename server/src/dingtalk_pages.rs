use rocket::request::Form;
use rocket_contrib::json::Json;
use serde::Serialize;

use dingtalk::*;
use data::*;
use data::models::*;
use super::pages::Session;

#[derive(FromForm, Debug)]
pub struct AuthForm {
    authCode: String,
}

#[derive(Serialize)]
pub struct DingTalkUser {
    userId: String,
    userName: String,
}

type JsonResult<T> = std::result::Result<Json<T>, Box<dyn std::error::Error>>; 

#[post("/dingtalk/login", data="<auth>")]
pub fn login(auth: Form<AuthForm>, session: Session) -> JsonResult<DingTalkUser> {

    let access_token = &get_token()?;
    let auth_code = &auth.authCode;

    let user = get_user_id(access_token, auth_code)?;

    let stored_user = match select_user(&user.userid, &user.userid)? {
        Some(stored_user) => {
            stored_user
        },
        None => {
            create_user(&user.userid, &user.userid, &user.username, false)?
        },
    };


    let user = DingTalkUser {
        userId: stored_user.username.clone(),
        userName: stored_user.display_name.clone(),
    };

    session.tap(|user_wrapper| {
        user_wrapper.user = Some(stored_user);
    });

    Ok(Json(user))
}

#[get("/dingtalk/assigned_projects")]
pub fn get_assigned_projects(session: Session) -> JsonResult<Vec<Project>> {
    session.tap( move | user_wrapper | {
        let user = &user_wrapper.user.as_ref().unwrap();
        let assigned_projects = select_assigned_projects(&user)?;
        Ok(Json(assigned_projects))
    })


}