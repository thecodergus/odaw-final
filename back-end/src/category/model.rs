use postgres::{Client, Error, Transaction};
use rocket::http::ext::IntoCollection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::get_client;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Category {
    id: Option<Uuid>,
    nome: String,
    id_documento: Option<Uuid>,
}

pub async fn find_category(id: &Uuid) -> Result<Category, Error> {
    match get_client().await {
        Ok(mut client) => {
            return match client.query_one("SELECT id, nome FROM categoria WHERE id='$1'", &[&id]) {
                Ok(row) => Ok(Category {
                    id: Some(id.clone()),
                    nome: row.get(1),
                    id_documento: Some(row.get(2)),
                }),
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

pub async fn find_categories_by_document(id_documento: &Uuid) -> Result<Vec<Category>, Error> {
    match get_client().await {
        Ok(mut client) => {
            return match client.query(
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
        Err(err) => Err(err),
    }
}

pub async fn delete_category(id: &Uuid) -> Result<(), Error> {
    match get_client().await {
        Ok(mut client) => match client.query_one("DELETE FROM categeria WHERE id = $1", &[id]) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

pub async fn delete_category_by_document(id_documento: &Uuid) -> Result<(), Error> {
    match get_client().await {
        Ok(mut client) => match client.query_one(
            "DELETE FROM categeria WHERE id_documento = $1",
            &[id_documento],
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

pub async fn register_categories(
    id_documento: &Uuid,
    categories: Vec<Category>,
) -> Result<(), Error> {
    let mut client: Client = match get_client().await {
        Ok(c) => c,
        Err(err) => return Err(err),
    };
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
    use rocket::tokio;

    use super::*;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_register_categories() {
        let document_id =
            Uuid::from_str("7209bb10-6a13-4224-b50e-a8992863c71b").expect("não é uuid");

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

        match register_categories(&document_id, categorias).await {
            Ok(_) => (),
            Err(err) => panic!("Deu ruim: {}", err),
        }
    }

    #[tokio::test]
    async fn test_find_categories_by_document() {
        let document_id =
            Uuid::from_str("7209bb10-6a13-4224-b50e-a8992863c71b").expect("não é uuid");

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

        match register_categories(&document_id, categorias).await {
            Ok(_) => (),
            Err(err) => panic!("Deu ruim: {}", err),
        }

        match find_categories_by_document(&document_id).await {
            Ok(_) => (),
            Err(err) => panic!("Deu merda: {}", err),
        }
    }

    #[tokio::test]
    async fn test_delete_category() {
        let document_id =
            Uuid::from_str("7209bb10-6a13-4224-b50e-a8992863c71b").expect("não é uuid");

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

        match register_categories(&document_id, categorias).await {
            Ok(_) => (),
            Err(err) => panic!("Deu ruim: {}", err),
        }

        let ret_cate = match find_categories_by_document(&document_id).await {
            Ok(v) => v,
            Err(err) => panic!("Deu merda: {}", err),
        };

        for c in ret_cate {
            match delete_category(&c.id.unwrap()).await {
                Ok(_) => (),
                Err(err) => panic!("Deu ruim: {}", err),
            }
        }
    }

    #[tokio::test]
    async fn test_delete_category_by_document() {
        let document_id =
            Uuid::from_str("7209bb10-6a13-4224-b50e-a8992863c71b").expect("não é uuid");

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

        match register_categories(&document_id, categorias).await {
            Ok(_) => (),
            Err(err) => panic!("Deu ruim: {}", err),
        }

        match delete_category_by_document(&document_id).await {
            Ok(_) => (),
            Err(err) => panic!("Deu merda: {}", err),
        }
    }
}
