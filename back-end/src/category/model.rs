use postgres::{Client, Error, Transaction};
use rocket::http::ext::IntoCollection;
use uuid::Uuid;

use crate::db::get_client;

pub struct Category {
    id: Option<Uuid>,
    nome: String,
    id_documento: Option<Uuid>,
}

pub fn find_category(id: Uuid) -> Result<Category, Error> {
    match get_client().query_one("SELECT id, nome FROM categoria WHERE id='$1'", &[&id]) {
        Ok(row) => Ok(Category {
            id: Some(id),
            nome: row.get(1),
            id_documento: Some(row.get(2)),
        }),
        Err(err) => Err(err),
    }
}

pub fn find_categories(id_documento: Uuid) -> Result<Vec<Category>, Error> {
    match get_client().query(
        "SELECT id, nome FROM categoria WHERE id_documento='$1'",
        &[&id_documento],
    ) {
        Ok(ret) => Ok(ret
            .into_iter()
            .map(|r| Category {
                id: r.get(0),
                nome: r.get(1),
                id_documento: Some(id_documento),
            })
            .collect()),
        Err(err) => Err(err),
    }
}

pub fn register_categories(id_documento: String, categories: Vec<Category>) -> Result<(), Error> {
    let mut client: Client = get_client();
    let mut transaction: Transaction = match client.transaction() {
        Ok(t) => t,
        Err(err) => panic!("Deu merda: {}", err),
    };

    for c in categories {
        match transaction.execute(
            "INSERT INTO categoria (nome, id_documento) VALUES ($1, $2);",
            &[&c.nome, &id_documento],
        ) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
    }

    return Ok(());
}
