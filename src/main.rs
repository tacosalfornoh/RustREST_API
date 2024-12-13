#[macro_use] extern crate rocket;

// ! MODULES
mod api;
mod utils;
// ! USES
use api::students::view_students;
use api::students::add_student;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/",routes![index, view_students, add_student])
}