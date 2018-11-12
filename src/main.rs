#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;
extern crate rocket_contrib;
extern crate time;

use rocket::request::Form;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;


#[derive(FromForm)]
struct Gasto {
    valor: f32,
    tipo: u8,
    descricao: String,
    idUsuario: u32,
}


#[get("/newCusto")]
fn new_custo() -> Template {
    Template::render("form", HashMap::<String, String>::new())
}

#[post("/cadastroCusto", data = "<custo>")]
fn cadastro_custo(custo: Form<Gasto>) -> Template {
    Template::render("form", HashMap::<String, String>::new())
}

#[get("/<file..>", rank = 10)] // set the rank higher than other endpoints
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/").join(file)).ok()
}


#[get("/")]
fn index() -> Template {
    Template::render("tables", HashMap::<String, String>::new())
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index, files, new_custo, cadastro_custo])
        .attach(Template::fairing())
        .launch();
}
