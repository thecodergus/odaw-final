#[macro_use]
extern crate rocket;
extern crate rocket_cors;

mod categoria;
mod db;
mod documento;
mod responses;
mod schema;
mod usuario;

use rocket::http::Method;

use rocket_cors::{
    AllowedHeaders,
    AllowedOrigins,
    Cors,
    CorsOptions, // 3.
    Error,       // 2.
};

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        // 4.
        "http://localhost:8080",
        "http://127.0.0.1:8080",
        "http://localhost:8000",
        "http://0.0.0.0:8000",
        "http://localhost:5173",
        "http://127.0.0.1:5173",
        "http://localhost:5173",
        "http://0.0.0.0:5173",
    ]);

    CorsOptions {
        // 5.
        allowed_origins,
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Patch,
            Method::Delete,
        ]
        .into_iter()
        .map(From::from)
        .collect(), // 1.
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin", // 6.
            "Content-Type",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

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
        .attach(make_cors())
}
