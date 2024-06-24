use postgres::{Client, Error, Transaction};
use rocket::http::ext::IntoCollection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::get_client;

#[derive(Clone, Serialize, Deserialize)]
pub struct Category {
    id: Option<Uuid>,
    nome: String,
    id_documento: Option<Uuid>,
}

pub fn find_category(id: &Uuid) -> Result<Category, Error> {
    match get_client().query_one("SELECT id, nome FROM categoria WHERE id='$1'", &[&id]) {
        Ok(row) => Ok(Category {
            id: Some(id.clone()),
            nome: row.get(1),
            id_documento: Some(row.get(2)),
        }),
        Err(err) => Err(err),
    }
}

pub fn find_categories_by_document(id_documento: &Uuid) -> Result<Vec<Category>, Error> {
    match get_client().query(
        "SELECT id, nome FROM categoria WHERE id_documento='$1'",
        &[&id_documento],
    ) {
        Ok(ret) => Ok(ret
            .into_iter()
            .map(|r| Category {
                id: r.get(0),
                nome: r.get(1),
                id_documento: Some(id_documento.clone()),
            })
            .collect()),
        Err(err) => Err(err),
    }
}

pub fn delete_category(id: &Uuid) -> Result<(), Error> {
    match get_client().query_one("DELETE FROM categeria WHERE id = $1", &[id]) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub fn delete_category_by_document(id_documento: &Uuid) -> Result<(), Error> {
    match get_client().query_one(
        "DELETE FROM categeria WHERE id_documento = $1",
        &[id_documento],
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub fn register_categories(id_documento: &Uuid, categories: Vec<Category>) -> Result<(), Error> {
    let mut client: Client = get_client();
    let mut transaction: Transaction = match client.transaction() {
        Ok(t) => t,
        Err(err) => return Err(err),
    };

    for c in categories {
        match transaction.execute(
            "INSERT INTO categoria (nome, id_documento) VALUES ($1, $2);",
            &[&c.nome, id_documento],
        ) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
    }

    transaction.commit()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn test_register_categories() {
        let document_id =
            Uuid::from_str("7b79914b-306a-419f-a1b7-1034db7edba2").expect("não é uuid");

        let categorias = vec![
            Category {
                id: None,
                nome: "Categoria 1".to_string(),
                id_documento: Some(document_id.clone()),
            },
            Category {
                id: None,
                nome: "Categoria 2".to_string(),
                id_documento: Some(document_id.clone()),
            },
            Category {
                id: None,
                nome: "Categoria 3".to_string(),
                id_documento: Some(document_id.clone()),
            },
        ];

        match register_categories(&document_id, categorias) {
            Ok(_) => (),
            Err(err) => panic!("Deu ruim: {}", err),
        }
    }

    fn test_find_categories_by_document() {
        let document_id =
            Uuid::from_str("7b79914b-306a-419f-a1b7-1034db7edba2").expect("não é uuid");

        let categorias = vec![
            Category {
                id: None,
                nome: "Categoria 1".to_string(),
                id_documento: Some(document_id.clone()),
            },
            Category {
                id: None,
                nome: "Categoria 2".to_string(),
                id_documento: Some(document_id.clone()),
            },
            Category {
                id: None,
                nome: "Categoria 3".to_string(),
                id_documento: Some(document_id.clone()),
            },
        ];

        match register_categories(&document_id, categorias) {
            Ok(_) => (),
            Err(err) => panic!("Deu ruim: {}", err),
        }

        match find_categories_by_document(&document_id) {
            Ok(_) => (),
            Err(err) => panic!("Deu merda: {}", err),
        }
    }

    fn test_delete_category() {
        let document_id =
            Uuid::from_str("7b79914b-306a-419f-a1b7-1034db7edba2").expect("não é uuid");

        let categorias = vec![
            Category {
                id: None,
                nome: "Categoria 1".to_string(),
                id_documento: Some(document_id.clone()),
            },
            Category {
                id: None,
                nome: "Categoria 2".to_string(),
                id_documento: Some(document_id.clone()),
            },
            Category {
                id: None,
                nome: "Categoria 3".to_string(),
                id_documento: Some(document_id.clone()),
            },
        ];

        match register_categories(&document_id, categorias) {
            Ok(_) => (),
            Err(err) => panic!("Deu ruim: {}", err),
        }

        let ret_cate = match find_categories_by_document(&document_id) {
            Ok(v) => v,
            Err(err) => panic!("Deu merda: {}", err),
        };

        for c in ret_cate {
            match delete_category(&c.id.unwrap()) {
                Ok(_) => (),
                Err(err) => panic!("Deu ruim: {}", err),
            }
        }
    }

    fn test_delete_category_by_document() {
        let document_id =
            Uuid::from_str("7b79914b-306a-419f-a1b7-1034db7edba2").expect("não é uuid");

        let categorias = vec![
            Category {
                id: None,
                nome: "Categoria 1".to_string(),
                id_documento: Some(document_id.clone()),
            },
            Category {
                id: None,
                nome: "Categoria 2".to_string(),
                id_documento: Some(document_id.clone()),
            },
            Category {
                id: None,
                nome: "Categoria 3".to_string(),
                id_documento: Some(document_id.clone()),
            },
        ];

        match register_categories(&document_id, categorias) {
            Ok(_) => (),
            Err(err) => panic!("Deu ruim: {}", err),
        }

        match delete_category_by_document(&document_id) {
            Ok(_) => (),
            Err(err) => panic!("Deu merda: {}", err),
        }
    }
}
