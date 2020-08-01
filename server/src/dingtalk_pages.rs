use rocket::request::Form;
use rocket_contrib::json::Json;
use serde::Serialize;

use dingtalk::*;
use data::*;
use data::models::*;
use crate::pages::Session;
use crate::requests::track_form::TrackForm;

#[derive(FromForm, Debug)]
pub struct AuthForm {
    authCode: String,
}

#[derive(Serialize)]
pub struct DingTalkUser {
    userId: String,
    userName: String,
}

#[derive(Serialize)]
pub struct DingTalkErrMsg {
    msg: String
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

#[post("/dingtalk/record/add", data = "<form>")]
pub fn add_record(session: Session, form: Form<TrackForm>) -> JsonResult<Vec<Tracking>> {
    session.tap( move | user_wrapper | {
        let user = &user_wrapper.user.as_ref().unwrap();
        let map = &form.map;
        let mut new_trackings = Vec::<(&str, i32, f32)>::new();
        for (project_id, recorded_time) in map {
            let project_id = project_id.parse::<i32>()?;
            let recorded_time = recorded_time.parse::<f32>()?;
            if recorded_time == 0.0 {
                continue;
            }
            new_trackings.push((&user.username, project_id, recorded_time));
        }
        let new_track = create_trackings(new_trackings)?;
        Ok(Json(new_track))
    })
}

#[get("/dingtalk/record")]
pub fn get_record(session: Session) -> JsonResult<Vec<(Project, Tracking)>> {
    session.tap( move | user_wrapper | {
        let user = &user_wrapper.user.as_ref().unwrap();
        let result = select_project_trackings_by_user(user)?;
        Ok(Json(result))
    })
}