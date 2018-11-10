extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;
use std::collections::HashMap;
use std::string::String;


#[get("/")]
pub fn index() -> Template {
    Template::render("tables", HashMap::<String, String>::new())
}
