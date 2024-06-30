use diesel::prelude::*;
use rocket::serde::json::Json; // Add this line

use crate::{responses::RespontaGenerica, schema::categorias};

use super::model::{Categoria, NovaCategoria};

#[post("/", format = "json", data = "<nova_categoria>")]
pub fn nova_categoria(nova_categoria: Json<NovaCategoria>) -> Json<RespontaGenerica> {
    let mut connection = crate::db::estabelecer_conexao();
    let categoria = nova_categoria.into_inner();

    let result = diesel::insert_into(categorias::table)
        .values(&categoria)
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
            mensagem: Some(format!("Erro ao inserir categoria: {:?}", err)),
        }),
    }
}

#[get("/<id_categoria>", format = "json")]
pub fn obter_categoria(id_categoria: String) -> Result<Json<Categoria>, Json<RespontaGenerica>> {
    let id_categoria = uuid::Uuid::parse_str(&id_categoria).unwrap();
    let mut connection = crate::db::estabelecer_conexao();

    let categoria = categorias::table
        .find(id_categoria)
        .first::<Categoria>(&mut connection);

    match categoria {
        Ok(categoria) => Ok(Json(categoria)),
        Err(err) => Err(Json(RespontaGenerica {
            status: "erro".to_string(),
            codigo: 500,
            mensagem: Some(format!("Erro ao obter categoria: {:?}", err)),
        })),
    }
}

#[get("/documento/<id_documento>", format = "json")]
pub fn obter_categorias(
    id_documento: String,
) -> Result<Json<Vec<Categoria>>, Json<RespontaGenerica>> {
    let id_documento = uuid::Uuid::parse_str(&id_documento).unwrap();
    let mut connection = crate::db::estabelecer_conexao();

    let categorias = categorias::table
        .filter(categorias::id_documento.eq(id_documento))
        .load::<Categoria>(&mut connection);

    match categorias {
        Ok(categorias) => Ok(Json(categorias)),
        Err(err) => Err(Json(RespontaGenerica {
            status: "erro".to_string(),
            codigo: 500,
            mensagem: Some(format!("Erro ao obter categorias: {:?}", err)),
        })),
    }
}

#[delete("/<id_categoria>", format = "json")]
pub fn deletar_categoria(id_categoria: String) -> Json<RespontaGenerica> {
    let id_categoria = uuid::Uuid::parse_str(&id_categoria).unwrap();
    let mut connection = crate::db::estabelecer_conexao();

    let result = diesel::delete(categorias::table.find(id_categoria)).execute(&mut connection);

    match result {
        Ok(_) => Json(RespontaGenerica {
            status: "sucesso".to_string(),
            codigo: 200,
            mensagem: None,
        }),
        Err(err) => Json(RespontaGenerica {
            status: "erro".to_string(),
            codigo: 500,
            mensagem: Some(format!("Erro ao deletar categoria: {:?}", err)),
        }),
    }
}

#[delete("/<id_documento>", format = "json")]
pub fn deletar_categorias(id_documento: String) -> Json<RespontaGenerica> {
    let id_documento = uuid::Uuid::parse_str(&id_documento).unwrap();
    let mut connection = crate::db::estabelecer_conexao();

    let result =
        diesel::delete(categorias::table.filter(categorias::id_documento.eq(id_documento)))
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
            mensagem: Some(format!("Erro ao deletar categorias: {:?}", err)),
        }),
    }
}
