#[macro_use]
extern crate rocket;

mod categoria;
mod db;
mod documento;
mod responses;
mod schema;
mod usuario;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api",
            routes![
                usuario::routes::novo_usuario,
                usuario::routes::atualizar_usuario
            ],
        )
        .mount("/api/login", routes![usuario::routes::fazer_login])
}
