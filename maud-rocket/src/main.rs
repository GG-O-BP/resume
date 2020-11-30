#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use maud::{html, Markup};
use std:: borrow::Cow;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
fn hello<'a>(name: Cow<'a, str>) -> Markup {
    html! {
        h1{ "Hello, " (name) "!" }
        p{ "Nice to meet you!" }
    }
}


fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
