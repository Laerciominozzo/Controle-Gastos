#![feature(plugin)]
#![plugin(rocket_codegen)]


extern crate rocket;
extern crate rocket_contrib;

use rocket::response::NamedFile;
use rocket_contrib::Template;
use std::path::Path;
use std::path::PathBuf;

mod controller;


#[get("/<file..>", rank = 10)] // set the rank higher than other endpoints
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/").join(file)).ok()
}


fn main() {
    rocket::ignite()
        .mount("/", routes![controller::table::index, files, controller::form::form])
        .attach(Template::fairing())
        .launch();
}
