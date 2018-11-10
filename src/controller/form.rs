extern crate rocket;
extern crate rocket_contrib;


use rocket_contrib::Template;
use std::collections::HashMap;
use std::string::String;

#[get("/form")]
pub fn form() -> Template {
    Template::render("form", HashMap::<String, String>::new())
}
