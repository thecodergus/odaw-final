use crate::{responses::RespontaGenerica, schema::usuarios};
use diesel::prelude::*;
use diesel::sql_types::Uuid;
use rocket::serde::json::Json;
use rocket::{http::hyper::server::conn, response::status}; // Add this line
use sha_crypt::{sha512_check, sha512_simple, Sha512Params};

use super::model::{AtualizarUsuario, CredenciaisUsuario, NovoUsuario, Usuario};

#[post("/", format = "json", data = "<novo_usuario>")]
pub fn novo_usuario(
    novo_usuario: Json<NovoUsuario>,
) -> Result<status::Accepted<Json<RespontaGenerica>>, status::Conflict<Json<RespontaGenerica>>> {
    let mut connection = crate::db::estabelecer_conexao();
    let mut usuario = novo_usuario.into_inner();

    let procurar = usuarios::table
        .filter(usuarios::email.eq(&usuario.email))
        .first::<Usuario>(&mut connection);

    if let Ok(_) = procurar {
        return Err(status::Conflict(Json(RespontaGenerica {
            status: "erro".to_string(),
            mensagem: Some(format!("Erro ao inserir usuário: email já cadastrado")),
        })));
    }

    // Criptografar senha
    let params = Sha512Params::new(10_000).expect("Erro ao criar parametros sha512");
    let senha_criptografada =
        sha512_simple(&usuario.senha, &params).expect("Erro ao criptografar senha");

    usuario.senha = &senha_criptografada;

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
        .first::<Usuario>(&mut connection);

    match usuario {
        Ok(usuario) => match sha512_check(&login.senha, &usuario.senha) {
            Ok(_) => Ok(status::Accepted(Json(usuario))),
            Err(_) => {
                return Err(status::NotFound(Json(RespontaGenerica {
                    status: "erro".to_string(),
                    mensagem: Some(format!(
                    "Erro ao fazer login: Nenhum usuario com este e-mail e senha foi encontrado!"
                )),
                })))
            }
        },
        Err(_) => Err(status::NotFound(Json(RespontaGenerica {
            status: "erro".to_string(),
            mensagem: Some(format!(
                "Erro ao fazer login: Nenhum usuario com este e-mail e senha foi encontrado!"
            )),
        }))),
    }
}

#[put("/", format = "json", data = "<atualizar_usuario>")]
pub fn atualizar_usuario(
    atualizar_usuario: Json<AtualizarUsuario>,
) -> Result<status::Accepted<Json<Usuario>>, status::Conflict<Json<RespontaGenerica>>> {
    let mut connection = crate::db::estabelecer_conexao();
    let mut usuario = atualizar_usuario.into_inner();

    // Criptografar senha caso tenha sido enviada
    if let Some(senha) = usuario.senha {
        let params = Sha512Params::new(10_000).expect("Erro ao criar parametros sha512");
        let senha_criptografada =
            sha512_simple(&senha, &params).expect("Erro ao criptografar senha");

        usuario.senha = Some(senha_criptografada);
    }

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

#[get("/<id_usuario>")]
pub fn buscar_usuario(
    id_usuario: String,
) -> Result<status::Accepted<Json<Usuario>>, status::NotFound<Json<RespontaGenerica>>> {
    let mut connection = crate::db::estabelecer_conexao();
    let id = uuid::Uuid::parse_str(&id_usuario).unwrap();

    let usuario = usuarios::table.find(id).first::<Usuario>(&mut connection);

    match usuario {
        Ok(usuario) => Ok(status::Accepted(Json(usuario))),
        Err(_) => Err(status::NotFound(Json(RespontaGenerica {
            status: "erro".to_string(),
            mensagem: Some(format!(
                "Erro ao buscar usuário: Nenhum usuário com este id foi encontrado!"
            )),
        }))),
    }
}
