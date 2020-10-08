#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::Data;
use std::fs;

#[get("/")]
fn index() -> &'static str {
    "Welcome to your very own server, but there is nothing at the main branch\n"
}

#[get("/bacon")]
fn bacon() -> String {
    let bacon_contents = fs::read_to_string("bacon.txt")
        .expect("Unable to open file");
    
    format!("{}\n", bacon_contents)
}

#[post("/upload", format = "plain", data = "<data>")]
fn upload(data: Data) -> Result<String, std::io::Error> {
    data.stream_to_file("/tmp/data.txt").map(|num_bytes| format!("Wrote {} bytes out to file", num_bytes))
}

#[get("/greetz/<name>/<age>")]
fn greetz(name: String, age: u8) -> String {
    format!("Greetz, {} year old named {}!", age, name)
}

#[get("/ofage/<name>/<age>")]
fn ofage(name: String, age: u8) -> String {
    if age > 18 {
        format!("Welcome, {}, your age {} means you can view this content!", name, age)
    }
    else {
        format!("Sorry, {}, you are not the right age, since you are only {} years old", name, age)
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index,greetz,upload,bacon,ofage])
    .launch();
}

