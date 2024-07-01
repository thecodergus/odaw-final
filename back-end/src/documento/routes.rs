use super::model::{AtualizarDocumento, Documento, NovoDocumento};
use crate::responses::RespontaGenerica;
use crate::schema::{categorias, documentos};
use diesel::prelude::*;
use rocket::response::status;
use rocket::serde::json::Json;

#[post("/", format = "json", data = "<documento>")]
pub fn criar_documento(
    documento: Json<NovoDocumento>,
) -> Result<status::Accepted<Json<RespontaGenerica>>, status::BadRequest<Json<RespontaGenerica>>> {
    let documento = documento.into_inner();
    let mut connection = crate::db::estabelecer_conexao();

    let result = diesel::insert_into(documentos::table)
        .values(&documento)
        .execute(&mut connection);

    match result {
        Ok(_) => Ok(status::Accepted(Json(RespontaGenerica {
            status: "sucesso".to_string(),
            mensagem: None,
        }))),
        Err(err) => Err(status::BadRequest(Json(RespontaGenerica {
            status: "erro".to_string(),
            mensagem: Some(format!("Erro ao inserir documento: {:?}", err)),
        }))),
    }
}

#[get("/<id_documento>", format = "json")]
pub fn obter_documento(
    id_documento: String,
) -> Result<status::Accepted<Json<Documento>>, status::NotFound<Json<RespontaGenerica>>> {
    let id_documento = uuid::Uuid::parse_str(&id_documento).unwrap();
    let mut connection = crate::db::estabelecer_conexao();

    let documento = documentos::table
        .find(id_documento)
        .first::<Documento>(&mut connection);

    match documento {
        Ok(documento) => Ok(status::Accepted(Json(documento))),
        Err(err) => Err(status::NotFound(Json(RespontaGenerica {
            status: "erro".to_string(),
            mensagem: Some(format!("Erro ao obter documento: {:?}", err)),
        }))),
    }
}

#[get("/usuario/<id_usuario>", format = "json")]
pub fn obter_documentos(
    id_usuario: String,
) -> Result<status::Accepted<Json<Vec<Documento>>>, status::NotFound<Json<RespontaGenerica>>> {
    let id_usuario = uuid::Uuid::parse_str(&id_usuario).unwrap();
    let mut connection = crate::db::estabelecer_conexao();

    let documentos = documentos::table
        .filter(documentos::id_usuario.eq_any(vec![id_usuario]))
        .load::<Documento>(&mut connection);

    match documentos {
        Ok(documentos) => Ok(status::Accepted(Json(documentos))),
        Err(err) => Err(status::NotFound(Json(RespontaGenerica {
            status: "erro".to_string(),
            mensagem: Some(format!("Erro ao obter documentos: {:?}", err)),
        }))),
    }
}

#[put("/", format = "json", data = "<documento>")]
pub fn atualizar_documento(
    documento: Json<AtualizarDocumento>,
) -> Result<status::Accepted<Json<RespontaGenerica>>, status::BadRequest<Json<RespontaGenerica>>> {
    let documento = documento.into_inner();
    let mut connection = crate::db::estabelecer_conexao();

    let result = diesel::update(documentos::table.find(documento.id))
        .set(&documento)
        .execute(&mut connection);

    match result {
        Ok(_) => Ok(status::Accepted(Json(RespontaGenerica {
            status: "sucesso".to_string(),
            mensagem: None,
        }))),
        Err(err) => Err(status::BadRequest(Json(RespontaGenerica {
            status: "erro".to_string(),
            mensagem: Some(format!("Erro ao atualizar documento: {:?}", err)),
        }))),
    }
}

#[delete("/<id_documento>", format = "json")]
pub fn deletar_documento(
    id_documento: String,
) -> Result<status::Accepted<Json<RespontaGenerica>>, status::NotFound<Json<RespontaGenerica>>> {
    let id_documento = uuid::Uuid::parse_str(&id_documento).unwrap();
    let mut connection = crate::db::estabelecer_conexao();

    // Deletar todas as categorias associadas ao documento
    let result =
        diesel::delete(categorias::table.filter(categorias::id_documento.eq(id_documento)))
            .execute(&mut connection);

    if let Err(err) = result {
        return Err(status::NotFound(Json(RespontaGenerica {
            status: "erro".to_string(),
            mensagem: Some(format!(
                "Erro ao deletar categorias associadas ao documento: {:?}",
                err
            )),
        })));
    }

    let result_ = diesel::delete(documentos::table.find(id_documento)).execute(&mut connection);

    match result_ {
        Ok(_) => Ok(status::Accepted(Json(RespontaGenerica {
            status: "sucesso".to_string(),
            mensagem: None,
        }))),
        Err(err) => Err(status::NotFound(Json(RespontaGenerica {
            status: "erro".to_string(),
            mensagem: Some(format!("Erro ao deletar documento: {:?}", err)),
        }))),
    }
}
