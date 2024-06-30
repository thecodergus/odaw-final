use crate::schema::usuarios;
use chrono::NaiveDate;
use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, Serialize, Deserialize, Selectable)]
#[serde(crate = "rocket::serde")]
pub struct Usuario {
    pub id: Uuid,
    pub nome: String,
    pub email: String,
    pub senha: String,
    pub data_de_nascimento: NaiveDate,
}

#[derive(Debug, Insertable, Queryable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "usuarios"]
pub struct NovoUsuario<'a> {
    pub nome: &'a str,
    pub email: &'a str,
    pub senha: &'a str,
    pub data_de_nascimento: NaiveDate,
}

#[derive(Deserialize, AsChangeset, Queryable)]
#[serde(crate = "rocket::serde")]
#[table_name = "usuarios"]
pub struct AtualizarUsuario {
    pub id: Uuid,
    pub nome: Option<String>,
    pub email: Option<String>,
    pub senha: Option<String>,
    pub data_de_nascimento: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CredenciaisUsuario {
    pub email: String,
    pub senha: String,
}
