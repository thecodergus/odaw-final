use crate::schema::usuarios;
use chrono::NaiveDate;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Usuario {
    pub id: Option<Uuid>,
    pub nome: String,
    pub email: String,
    pub senha: Option<String>,
    pub data_de_nascimento: NaiveDate,
}

#[derive(Debug, Insertable, Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "usuarios"]
pub struct NovoUsuario<'a> {
    pub nome: &'a str,
    pub email: &'a str,
    pub senha: &'a str,
    pub data_de_nascimento: NaiveDate,
}

#[derive(Deserialize, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "usuarios"]
pub struct AtualizarUsuario {
    pub id: Uuid,
    pub nome: Option<String>,
    pub email: Option<String>,
    pub senha: Option<String>,
    pub data_de_nascimento: Option<NaiveDate>,
}
