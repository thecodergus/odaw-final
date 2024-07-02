use diesel::prelude::*;
use rocket::response::status;
use rocket::serde::json::Json; // Add this line

use crate::{responses::RespontaGenerica, schema::categorias};

use super::model::{Categoria, NovaCategoria};

#[post("/", format = "json", data = "<nova_categoria>")]
pub fn nova_categoria(
    nova_categoria: Json<NovaCategoria>,
) -> Result<status::Accepted<Json<RespontaGenerica>>, status::BadRequest<Json<RespontaGenerica>>> {
    let mut connection = crate::db::estabelecer_conexao();
    let categoria = nova_categoria.into_inner();

    //Verificar se já existe uma categoria com o mesmo nome
    let categoria_existente = categorias::table
        .filter(categorias::nome.eq(&categoria.nome))
        .first::<Categoria>(&mut connection);

    match categoria_existente {
        Ok(_) => {
            return Ok(status::Accepted(Json(RespontaGenerica {
                status: "sucesso".to_string(),
                mensagem: None,
            }))
            .into());
        }
        Err(_) => (),
    }

    let result = diesel::insert_into(categorias::table)
        .values(&categoria)
        .execute(&mut connection);

    match result {
        Ok(_) => Ok(status::Accepted(Json(RespontaGenerica {
            status: "sucesso".to_string(),
            mensagem: None,
        }))),
        Err(err) => Err(status::BadRequest(Json(RespontaGenerica {
            status: "erro".to_string(),
            mensagem: Some(format!("Erro ao inserir categoria: {:?}", err)),
        }))),
    }
}

#[get("/<id_categoria>", format = "json")]
pub fn obter_categoria(
    id_categoria: String,
) -> Result<status::Accepted<Json<Categoria>>, status::NotFound<Json<RespontaGenerica>>> {
    let id_categoria = uuid::Uuid::parse_str(&id_categoria).unwrap();
    let mut connection = crate::db::estabelecer_conexao();

    let categoria = categorias::table
        .find(id_categoria)
        .first::<Categoria>(&mut connection);

    match categoria {
        Ok(categoria) => Ok(status::Accepted(Json(categoria))),
        Err(err) => Err(status::NotFound(Json(RespontaGenerica {
            status: "erro".to_string(),
            mensagem: Some(format!("Erro ao obter categoria: {:?}", err)),
        }))),
    }
}

#[get("/documento/<id_documento>", format = "json")]
pub fn obter_categorias(
    id_documento: String,
) -> Result<status::Accepted<Json<Vec<Categoria>>>, status::NotFound<Json<RespontaGenerica>>> {
    let id_documento = uuid::Uuid::parse_str(&id_documento).unwrap();
    let mut connection = crate::db::estabelecer_conexao();

    let categorias = categorias::table
        .filter(categorias::id_documento.eq(id_documento))
        .load::<Categoria>(&mut connection);

    match categorias {
        Ok(categorias) => Ok(status::Accepted(Json(categorias))),
        Err(err) => Err(status::NotFound(Json(RespontaGenerica {
            status: "erro".to_string(),
            mensagem: Some(format!("Erro ao obter categorias: {:?}", err)),
        }))),
    }
}

#[delete("/<id_categoria>")]
pub fn deletar_categoria(
    id_categoria: String,
) -> Result<status::Accepted<Json<RespontaGenerica>>, status::NotFound<Json<RespontaGenerica>>> {
    let id_categoria = uuid::Uuid::parse_str(&id_categoria).unwrap();
    let mut connection = crate::db::estabelecer_conexao();

    let result = diesel::delete(categorias::table.find(id_categoria)).execute(&mut connection);

    match result {
        Ok(_) => Ok(status::Accepted(Json(RespontaGenerica {
            status: "sucesso".to_string(),
            mensagem: None,
        }))),
        Err(err) => Err(status::NotFound(Json(RespontaGenerica {
            status: "erro".to_string(),
            mensagem: Some(format!("Erro ao deletar categoria: {:?}", err)),
        }))),
    }
}

#[delete("/documento/<id_documento>")]
pub fn deletar_categorias(
    id_documento: String,
) -> Result<status::Accepted<Json<RespontaGenerica>>, status::NotFound<Json<RespontaGenerica>>> {
    let id_documento = uuid::Uuid::parse_str(&id_documento).unwrap();
    let mut connection = crate::db::estabelecer_conexao();

    let result =
        diesel::delete(categorias::table.filter(categorias::id_documento.eq(id_documento)))
            .execute(&mut connection);

    match result {
        Ok(_) => Ok(status::Accepted(Json(RespontaGenerica {
            status: "sucesso".to_string(),
            mensagem: None,
        }))),
        Err(err) => Err(status::NotFound(Json(RespontaGenerica {
            status: "erro".to_string(),
            mensagem: Some(format!("Erro ao deletar categorias: {:?}", err)),
        }))),
    }
}
