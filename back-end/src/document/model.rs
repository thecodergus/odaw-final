use crate::db::get_client;
use chrono::{Duration, NaiveDate};
use postgres::types::ToSql;
use postgres::Error;
use rust_decimal::Decimal;
use uuid::Uuid;

pub enum Type {
    CONTA,
    RECEITA,
}

impl Type {
    fn to_string(&self) -> String {
        match self {
            Type::CONTA => "CONTA".to_string(),
            Type::RECEITA => "receita".to_string(),
        }
    }
}

pub struct Document {
    id: Option<Uuid>,
    tipo_documento: Type,
    valor: Decimal,
    data: NaiveDate,
    descricao: String,
    id_usuario: Uuid,
}

// Function to insert a document into the database
pub fn insert_document(doc: &Document) -> Result<(), Error> {
    match get_client().execute(
        &format!("INSERT INTO documento (tipo_de_documento, valor, data, descricao, id_usuario) VALUES ('{}', '{}', $1, $2, $3)", doc.tipo_documento.to_string(), doc.valor.to_string()),
        &[&doc.data, &doc.descricao, &doc.id_usuario],
    ){
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}

// Function to update a document in the database
pub fn update_document(doc: &Document) -> Result<(), Error> {
    match get_client().execute(
        "UPDATE documento SET tipo_de_documento = $1, valor = $2, data = $3, descricao = $4, id_usuario = $5 WHERE id = $6",
        &[&doc.tipo_documento.to_string(), &doc.valor.to_string(), &doc.data, &doc.descricao, &doc.id_usuario],
    ){
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}

// Function to delete a document from the database
pub fn delete_document(id: String) -> Result<(), Error> {
    match get_client().execute("DELETE FROM documento WHERE id = $1", &[&id]) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_insert_document() {
        let doc = Document {
            id: None,
            tipo_documento: Type::RECEITA,
            valor: Decimal::from(100),
            data: NaiveDate::from_ymd_opt(2022, 1, 1).expect("Invalid date"),
            descricao: "Test document".to_string(),
            id_usuario: Uuid::from_str("99d9cc6f-a1f9-4f51-9b39-2a49e1a2323b").expect("não é uuid"),
        };

        match insert_document(&doc) {
            Ok(_) => (),
            Err(err) => panic!("Deu ruim: {}", err),
        }
    }

    // #[test]
    // fn test_update_document() {
    //     let doc = Document {
    //         id: None,
    //         tipo_documento: Type::Revenue,
    //         valor: Decimal::from(200),
    //         data: NaiveDate::from_ymd_opt(2022, 2, 2).expect("Invalid date"),
    //         descricao: "Updated document".to_string(),
    //         id_usuario: "user456".to_string(),
    //     };

    //     let result = update_document(&doc);
    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn test_delete_document() {
    //     let doc = Document {
    //         id: None,
    //         tipo_documento: Type::Revenue,
    //         valor: Decimal::from(200),
    //         data: NaiveDate::from_ymd_opt(2022, 2, 2).expect("Invalid date"),
    //         descricao: "Updated document".to_string(),
    //         id_usuario: "user456".to_string(),
    //     };

    //     let _ = insert_document(&doc);
    //     // let doc_2 = find

    //     let id = "document_id".to_string();

    //     let result = delete_document(id);
    //     assert!(result.is_ok());
    // }
}
