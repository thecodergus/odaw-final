#[macro_use]
extern crate rocket;

mod category;
mod db;
mod document;
mod responses;
mod user;

use chrono::NaiveDate;
use rocket::{routes, Build, Rocket};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/user", routes![user::routes::create_user])
}
