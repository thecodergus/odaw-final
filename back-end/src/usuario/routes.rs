use crate::{responses::RespontaGenerica, schema::usuarios};
use diesel::prelude::*;
use rocket::response::status;
use rocket::serde::json::Json; // Add this line

use super::model::{AtualizarUsuario, CredenciaisUsuario, NovoUsuario, Usuario};

#[post("/", format = "json", data = "<novo_usuario>")]
pub fn novo_usuario(
    novo_usuario: Json<NovoUsuario>,
) -> Result<status::Accepted<Json<RespontaGenerica>>, status::Conflict<Json<RespontaGenerica>>> {
    let mut connection = crate::db::estabelecer_conexao();
    let usuario = novo_usuario.into_inner();

    let result = diesel::insert_into(usuarios::table)
        .values(&usuario)
        .execute(&mut connection);

    match result {
        Ok(_) => Ok(status::Accepted(Json(RespontaGenerica {
            status: "sucesso".to_string(),
            mensagem: None,
        }))),
        Err(err) => Err(status::Conflict(Json(RespontaGenerica {
            status: "erro".to_string(),
            mensagem: Some(format!("Erro ao inserir usuário: {:?}", err)),
        }))),
    }
}

#[post("/", format = "json", data = "<credenciais>")]
pub fn fazer_login(
    credenciais: Json<CredenciaisUsuario>,
) -> Result<status::Accepted<Json<Usuario>>, status::NotFound<Json<RespontaGenerica>>> {
    let mut connection = crate::db::estabelecer_conexao();
    let login = credenciais.into_inner();

    let usuario = usuarios::table
        .filter(usuarios::email.eq(&login.email))
        .filter(usuarios::senha.eq(&login.senha))
        .first::<Usuario>(&mut connection);

    match usuario {
        Ok(usuario) => Ok(status::Accepted(Json(usuario))),
        Err(err) => Err(status::NotFound(Json(RespontaGenerica {
            status: "erro".to_string(),
            mensagem: Some(format!("Erro ao fazer login: {:?}", err)),
        }))),
    }
}

#[patch("/", format = "json", data = "<atualizar_usuario>")]
pub fn atualizar_usuario(
    atualizar_usuario: Json<AtualizarUsuario>,
) -> Result<status::Accepted<Json<Usuario>>, status::Conflict<Json<RespontaGenerica>>> {
    let mut connection = crate::db::estabelecer_conexao();
    let usuario = atualizar_usuario.into_inner();

    let result = diesel::update(usuarios::table.find(usuario.id))
        .set(&usuario)
        .get_result::<Usuario>(&mut connection);

    match result {
        Ok(usuario) => Ok(status::Accepted(Json(usuario))),
        Err(err) => Err(status::Conflict(Json(RespontaGenerica {
            status: "erro".to_string(),
            mensagem: Some(format!("Erro ao atualizar usuário: {:?}", err)),
        }))),
    }
}
