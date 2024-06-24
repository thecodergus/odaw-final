#[macro_use]
extern crate rocket;

mod category;
mod db;
mod document;
mod user;

use rocket::{routes, Build, Rocket};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/user", routes![user::routes::create_user])
}
