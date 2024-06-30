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
            "/api/usuario",
            routes![
                usuario::routes::novo_usuario,
                usuario::routes::atualizar_usuario
            ],
        )
        .mount("/api/login", routes![usuario::routes::fazer_login])
        .mount(
            "/api/documento",
            routes![
                documento::routes::criar_documento,
                documento::routes::obter_documento,
                documento::routes::obter_documentos,
                documento::routes::atualizar_documento,
                documento::routes::deletar_documento
            ],
        )
        .mount(
            "/api/categoria",
            routes![
                categoria::routes::nova_categoria,
                categoria::routes::obter_categorias,
                categoria::routes::obter_categoria
            ],
        )
}
