use crate::db::get_client;
use chrono::{Duration, NaiveDate};
use postgres::types::ToSql;
use postgres::{Error, Row};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Clone)]
pub enum Type {
    CONTA,
    RECEITA,
}

impl Type {
    fn to_string(&self) -> String {
        match self {
            Type::CONTA => "conta".to_string(),
            Type::RECEITA => "receita".to_string(),
        }
    }

    fn from_str(t: String) -> Type {
        if t.to_lowercase() == "conta" {
            return Type::CONTA;
        } else {
            return Type::RECEITA;
        }
    }
}

#[derive(Clone)]
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
        &format!("UPDATE documento SET tipo_de_documento = '{}', valor = '{}', data = $1, descricao = $2 WHERE id = $3", doc.tipo_documento.to_string(), doc.valor.to_string()),
        &[&doc.data, &doc.descricao, &doc.id],
    ){
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}

// Function to delete a document from the database
pub fn delete_document(id: &Uuid) -> Result<(), Error> {
    match get_client().execute("DELETE FROM documento WHERE id = $1", &[&id]) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub fn find_document(id: &Uuid) -> Result<Document, Error> {
    match get_client().query_one("SELECT id, CAST(tipo_de_documento AS VARCHAR), CAST(valor AS VARCHAR), data, descricao, id_usuario FROM documento WHERE id = $1" , &[id]){
        Ok(ret) => Ok(row_to_doc(ret)),
        Err(err) => Err(err)
    }
}

pub fn find_by_user(id: &Uuid) -> Result<Vec<Document>, Error> {
    match get_client().query("SELECT id, CAST(tipo_de_documento AS VARCHAR), CAST(valor AS VARCHAR), data, descricao, id_usuario FROM documento WHERE id_usuario = $1", &[id]) {
        Ok(ret) => Ok(ret.into_iter().map(row_to_doc).collect()),
        Err(err) => Err(err)
    }
}

fn row_to_doc(ret: Row) -> Document {
    Document {
        id: Some(ret.get("id")),
        data: ret.get("data"),
        descricao: ret.get("descricao"),
        tipo_documento: Type::from_str(ret.get("tipo_de_documento")),
        valor: Decimal::from_str_exact(ret.get("valor")).expect("Ta errado esse valor"),
        id_usuario: ret.get("id_usuario"),
    }
}

#[cfg(test)]
mod tests {
    use rocket::futures::SinkExt;

    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_insert_document() {
        let doc = Document {
            id: None,
            tipo_documento: Type::RECEITA,
            valor: Decimal::from(100),
            data: NaiveDate::from_ymd_opt(2022, 1, 1).expect("Invalid date"),
            descricao: "Test document".to_string(),
            id_usuario: Uuid::from_str("01fb7f56-0f79-457d-92d5-d5e41045b5f2").expect("não é uuid"),
        };

        match insert_document(&doc) {
            Ok(_) => (),
            Err(err) => panic!("Deu ruim: {}", err),
        }
    }

    #[test]
    fn test_find_document_by_user() {
        match find_by_user(
            &Uuid::from_str("01fb7f56-0f79-457d-92d5-d5e41045b5f2").expect("não é uuid"),
        ) {
            Ok(_) => (),
            Err(err) => panic!("Deu ruim: {}", err),
        }
    }

    #[test]
    fn test_find_document() {
        let doc = Document {
            id: None,
            tipo_documento: Type::RECEITA,
            valor: Decimal::from(100),
            data: NaiveDate::from_ymd_opt(2022, 1, 1).expect("Invalid date"),
            descricao: "Test document".to_string(),
            id_usuario: Uuid::from_str("01fb7f56-0f79-457d-92d5-d5e41045b5f2").expect("não é uuid"),
        };

        match insert_document(&doc) {
            Ok(_) => (),
            Err(err) => panic!("Deu ruim: {}", err),
        }

        let doc_2 = match find_by_user(
            &Uuid::from_str("01fb7f56-0f79-457d-92d5-d5e41045b5f2").expect("não é uuid"),
        ) {
            Ok(v) => v.clone(),
            Err(err) => panic!("Deu ruim: {}", err),
        };

        for d in doc_2 {
            match find_document(&d.id.unwrap()) {
                Ok(_) => (),
                Err(err) => panic!("Deu ruim: {}", err),
            }
        }
    }

    #[test]
    fn test_update_document() {
        let doc = Document {
            id: None,
            tipo_documento: Type::RECEITA,
            valor: Decimal::from(100),
            data: NaiveDate::from_ymd_opt(2022, 1, 1).expect("Invalid date"),
            descricao: "Test document".to_string(),
            id_usuario: Uuid::from_str("01fb7f56-0f79-457d-92d5-d5e41045b5f2").expect("não é uuid"),
        };

        match insert_document(&doc) {
            Ok(_) => (),
            Err(err) => panic!("Deu ruim: {}", err),
        }

        let doc_2 = match find_by_user(
            &Uuid::from_str("01fb7f56-0f79-457d-92d5-d5e41045b5f2").expect("não é uuid"),
        ) {
            Ok(v) => v.clone(),
            Err(err) => panic!("Deu ruim: {}", err),
        };

        match doc_2.first() {
            Some(d) => {
                let mut d2 = d.clone();
                d2.descricao = "Descrição alterada".to_string();

                match update_document(&d2) {
                    Ok(_) => (),
                    Err(err) => panic!("Deu ruim: {}", err),
                }
            }
            None => panic!("Deu ruim"),
        }
    }

    #[test]
    fn test_delete_document() {
        let doc = Document {
            id: None,
            tipo_documento: Type::RECEITA,
            valor: Decimal::from(100),
            data: NaiveDate::from_ymd_opt(2022, 1, 1).expect("Invalid date"),
            descricao: "Test document".to_string(),
            id_usuario: Uuid::from_str("01fb7f56-0f79-457d-92d5-d5e41045b5f2").expect("não é uuid"),
        };

        match insert_document(&doc) {
            Ok(_) => (),
            Err(err) => panic!("Deu ruim: {}", err),
        }

        let doc_2 = match find_by_user(
            &Uuid::from_str("01fb7f56-0f79-457d-92d5-d5e41045b5f2").expect("não é uuid"),
        ) {
            Ok(v) => v.clone(),
            Err(err) => panic!("Deu ruim: {}", err),
        };

        match doc_2.first() {
            Some(d) => {
                let _ = delete_document(&d.id.unwrap());
            }
            None => panic!("Nada encontrado"),
        }
    }
}
