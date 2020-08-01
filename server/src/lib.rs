#![feature(proc_macro_hygiene, decl_macro, never_type)]
#[macro_use] extern crate rocket;

pub mod pages;
pub mod dingtalk_pages;

pub mod requests {
    pub mod login_form;
    pub mod track_form;
}