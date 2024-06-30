use diesel::prelude::*;
use rocket::serde::json::Json; // Add this line

use crate::{responses::RespontaGenerica, schema::usuarios};

use super::model::{AtualizarUsuario, CredenciaisUsuario, NovoUsuario, Usuario};

#[post("/", format = "json", data = "<novo_usuario>")]
pub fn novo_usuario(novo_usuario: Json<NovoUsuario>) -> Json<RespontaGenerica> {
    let mut connection = crate::db::estabelecer_conexao();
    let usuario = novo_usuario.into_inner();

    let result = diesel::insert_into(usuarios::table)
        .values(&usuario)
        .execute(&mut connection);

    match result {
        Ok(_) => Json(RespontaGenerica {
            status: "sucesso".to_string(),
            codigo: 201,
            mensagem: None,
        }),
        Err(err) => Json(RespontaGenerica {
            status: "erro".to_string(),
            codigo: 500,
            mensagem: Some(format!("Erro ao inserir usuário: {:?}", err)),
        }),
    }
}

#[post("/", format = "json", data = "<credenciais>")]
pub fn fazer_login(
    credenciais: Json<CredenciaisUsuario>,
) -> Result<Json<Usuario>, Json<RespontaGenerica>> {
    let mut connection = crate::db::estabelecer_conexao();
    let login = credenciais.into_inner();

    let usuario = usuarios::table
        .filter(usuarios::email.eq(&login.email))
        .filter(usuarios::senha.eq(&login.senha))
        .first::<Usuario>(&mut connection);

    match usuario {
        Ok(usuario) => Ok(Json(usuario)),
        Err(err) => Err(Json(RespontaGenerica {
            status: "erro".to_string(),
            codigo: 500,
            mensagem: Some(format!("Erro ao fazer login: {:?}", err)),
        })),
    }
}

#[patch("/", format = "json", data = "<atualizar_usuario>")]
pub fn atualizar_usuario(
    atualizar_usuario: Json<AtualizarUsuario>,
) -> Result<Json<Usuario>, Json<RespontaGenerica>> {
    let mut connection = crate::db::estabelecer_conexao();
    let usuario = atualizar_usuario.into_inner();

    let result = diesel::update(usuarios::table.find(usuario.id))
        .set(&usuario)
        .get_result::<Usuario>(&mut connection);

    match result {
        Ok(usuario) => Ok(Json(usuario)),
        Err(err) => Err(Json(RespontaGenerica {
            status: "erro".to_string(),
            codigo: 500,
            mensagem: Some(format!("Erro ao atualizar usuário: {:?}", err)),
        })),
    }
}
