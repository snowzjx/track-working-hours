#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::templates::Template;

use server::pages;
use server::pages::Session;

use server::dingtalk_pages;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            pages::index,
            pages::login,
            pages::post_login,
            pages::logout,
            pages::tracking,
            pages::post_tracking,
            pages::del_tracking,
            pages::error,
            pages::forget,
            pages::post_forget,
            pages::admin,
            pages::csv,
            dingtalk_pages::login,
            dingtalk_pages::get_assigned_projects,
        ])
        .attach(Template::fairing())
        .attach(Session::fairing())
        .launch();
}