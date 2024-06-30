use diesel::prelude::Queryable;
use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Documento {
    pub id: Uuid,
    pub tipo_de_documento: String,
    pub valor: f64,
    pub data: chrono::NaiveDate,
    pub descricao: String,
    pub usuario_id: Uuid,
}
