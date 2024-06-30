#[macro_use]
extern crate rocket;

mod categoria;
mod db;
mod documento;
mod schema;
mod usuario;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![])
}
