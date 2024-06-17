use crate::db::get_client;
use chrono::{Duration, NaiveDate};
use postgres::Error;
use rust_decimal::Decimal;

pub enum Type {
    Bill,
    Revenue,
}

impl Type {
    fn to_string(&self) -> String {
        match self {
            Type::Bill => "CONTA".to_string(),
            Type::Revenue => "RECEITA".to_string(),
        }
    }
}

pub struct Document {
    id: String,
    tipo_documento: Type,
    valor: Decimal,
    data: NaiveDate,
    descricao: String,
    id_usuario: String,
}

// Function to insert a document into the database
pub fn insert_document(doc: &Document) -> Result<(), Error> {
    match get_client().execute(
        "INSERT INTO documents (id, tipo_documento, valor, data, descricao, id_usuario) VALUES ($1, $2, $3, $4, $5, $6)",
        &[&doc.tipo_documento.to_string(), &doc.valor.to_string(), &doc.data.format("%Y-%m-%d").to_string(), &doc.descricao, &doc.id_usuario],
    ){
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}

// Function to update a document in the database
pub fn update_document(doc: &Document) -> Result<(), Error> {
    match get_client().execute(
        "UPDATE documents SET tipo_documento = $1, valor = $2, data = $3, descricao = $4, id_usuario = $5 WHERE id = $6",
        &[&doc.tipo_documento.to_string(), &doc.valor.to_string(), &doc.data.format("%Y-%m-%d").to_string(), &doc.descricao, &doc.id_usuario],
    ){
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}

// Function to delete a document from the database
pub fn delete_document(id: String) -> Result<(), Error> {
    match get_client().execute("DELETE FROM documents WHERE id = $1", &[&id]) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
