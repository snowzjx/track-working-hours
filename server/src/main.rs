#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::templates::Template;

use server::pages;
use server::pages::Session;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            pages::index,
            pages::login,
            pages::post_login,
            pages::logout,
            pages::tracking,
            pages::post_tracking,
            pages::error,
        ])
        .attach(Template::fairing())
        .attach(Session::fairing())
        .launch();
}