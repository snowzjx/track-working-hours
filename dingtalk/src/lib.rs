extern crate reqwest;

use std::error;
use std::fmt;
use std::env;

use dotenv::dotenv;

#[derive(Debug, Clone)]
struct DingTalkErr;

impl fmt::Display for DingTalkErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DingTalk Error")
    }
}

impl error::Error for DingTalkErr {}

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn get_token() -> Result<String> {
    dotenv().ok();

    let OAPI_HOST: String = env::var("OAPI_HOST").expect("OAPI_HOST must be set");
    let APP_KEY: String = env::var("APP_KEY").expect("APP_KEY must be set");
    let APP_SECRET: String = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let request_url = format!("{OAPI_HOST}/gettoken?appkey={APP_KEY}&appsecret={APP_SECRET}",
        OAPI_HOST = OAPI_HOST,
        APP_KEY = APP_KEY,
        APP_SECRET = APP_SECRET);

    let response_text = reqwest::blocking::get(&request_url)?.text()?;
    let parsed_result = json::parse(&response_text)?;

    let errcode = &parsed_result["errcode"];

    if errcode != 0 {
        return Err(DingTalkErr.into())
    }

    let access_token = parsed_result["access_token"].to_string();
    
    Ok(access_token)
}

pub struct User {
    pub userid: String,
    pub username: String,
}

pub fn get_user_id<'a>(access_token: &'a str, auth_code: &'a str) -> Result<User> {
    dotenv().ok();
    let OAPI_HOST: String = env::var("OAPI_HOST").expect("OAPI_HOST must be set");

    let request_url = format!("{OAPI_HOST}/user/getuserinfo?access_token={ACCESS_TOKEN}&code={CODE}",
        OAPI_HOST = OAPI_HOST,
        ACCESS_TOKEN = access_token,
        CODE = auth_code);
    
    let response_text = reqwest::blocking::get(&request_url)?.text()?;
    let parsed_result = json::parse(&response_text)?;

    let errcode = &parsed_result["errcode"];

    if errcode != 0 {
        return Err(DingTalkErr.into())
    }

    let userid = parsed_result["userid"].to_string();
    let username = parsed_result["name"].to_string();

    let user = User {
        userid: userid,
        username: username,
    };
    
    Ok(user)
}