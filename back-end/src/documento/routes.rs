use std::str::FromStr;

use super::model::{AtualizarDocumento, Documento, NovoDocumento};

use crate::responses::RespontaGenerica;
use crate::schema::{categorias, documentos};
use diesel::prelude::*;
use rocket::serde::json::Json;

#[post("/", format = "json", data = "<documento>")]
pub fn criar_documento(documento: Json<NovoDocumento>) -> Json<RespontaGenerica> {
    let documento = documento.into_inner();
    let mut connection = crate::db::estabelecer_conexao();

    let result = diesel::insert_into(documentos::table)
        .values(&documento)
        .execute(&mut connection);

    match result {
        Ok(_) => Json(RespontaGenerica {
            status: "sucesso".to_string(),
            codigo: 200,
            mensagem: None,
        }),
        Err(err) => Json(RespontaGenerica {
            status: "erro".to_string(),
            codigo: 500,
            mensagem: Some(format!("Erro ao inserir documento: {:?}", err)),
        }),
    }
}

#[get("/<id_documento>", format = "json")]
pub fn obter_documento(id_documento: String) -> Result<Json<Documento>, Json<RespontaGenerica>> {
    let id_documento = uuid::Uuid::parse_str(&id_documento).unwrap();
    let mut connection = crate::db::estabelecer_conexao();

    let documento = documentos::table
        .find(id_documento)
        .first::<Documento>(&mut connection);

    match documento {
        Ok(documento) => Ok(Json(documento)),
        Err(err) => Err(Json(RespontaGenerica {
            status: "erro".to_string(),
            codigo: 500,
            mensagem: Some(format!("Erro ao obter documento: {:?}", err)),
        })),
    }
}

#[get("/usuario/<id_usuario>", format = "json")]
pub fn obter_documentos(
    id_usuario: String,
) -> Result<Json<Vec<Documento>>, Json<RespontaGenerica>> {
    let id_usuario = uuid::Uuid::parse_str(&id_usuario).unwrap();
    let mut connection = crate::db::estabelecer_conexao();

    let documentos = documentos::table
        .filter(documentos::id_usuario.eq_any(vec![id_usuario]))
        .load::<Documento>(&mut connection);

    match documentos {
        Ok(documentos) => Ok(Json(documentos)),
        Err(err) => Err(Json(RespontaGenerica {
            status: "erro".to_string(),
            codigo: 500,
            mensagem: Some(format!("Erro ao obter documentos: {:?}", err)),
        })),
    }
}

#[put("/", format = "json", data = "<documento>")]
pub fn atualizar_documento(documento: Json<AtualizarDocumento>) -> Json<RespontaGenerica> {
    let documento = documento.into_inner();
    let mut connection = crate::db::estabelecer_conexao();

    let result = diesel::update(documentos::table.find(documento.id))
        .set(&documento)
        .execute(&mut connection);

    match result {
        Ok(_) => Json(RespontaGenerica {
            status: "sucesso".to_string(),
            codigo: 200,
            mensagem: None,
        }),
        Err(err) => Json(RespontaGenerica {
            status: "erro".to_string(),
            codigo: 500,
            mensagem: Some(format!("Erro ao atualizar documento: {:?}", err)),
        }),
    }
}

#[delete("/<id_documento>", format = "json")]
pub fn deletar_documento(id_documento: String) -> Json<RespontaGenerica> {
    let id_documento = uuid::Uuid::parse_str(&id_documento).unwrap();
    let mut connection = crate::db::estabelecer_conexao();

    // Deletar todas as categorias associadas ao documento
    let result =
        diesel::delete(categorias::table.filter(categorias::id_documento.eq(id_documento)))
            .execute(&mut connection);

    if let Err(err) = result {
        return Json(RespontaGenerica {
            status: "erro".to_string(),
            codigo: 500,
            mensagem: Some(format!(
                "Erro ao deletar categorias associadas ao documento: {:?}",
                err
            )),
        });
    }

    let result_ = diesel::delete(documentos::table.find(id_documento)).execute(&mut connection);

    match result_ {
        Ok(_) => Json(RespontaGenerica {
            status: "sucesso".to_string(),
            codigo: 200,
            mensagem: None,
        }),
        Err(err) => Json(RespontaGenerica {
            status: "erro".to_string(),
            codigo: 500,
            mensagem: Some(format!("Erro ao deletar documento: {:?}", err)),
        }),
    }
}
