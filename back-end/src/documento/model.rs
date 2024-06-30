use crate::schema::documentos;
use bigdecimal::BigDecimal;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};
use diesel::serialize::{self, ToSql};
use diesel_derive_enum::DbEnum;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, DbEnum, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[ExistingTypePath = "crate::schema::sql_types::DocumentoTipo"]
pub enum TipoDocumento {
    CONTA,
    RECEITA,
}

#[derive(Debug, Serialize, Queryable, Selectable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(check_for_backend(Pg))]
pub struct Documento {
    pub id: Uuid,
    pub tipo_de_documento: TipoDocumento,
    pub valor: BigDecimal,
    pub data: chrono::NaiveDate,
    pub descricao: Option<String>,
    pub id_usuario: Option<Uuid>,
}

#[derive(Debug, Insertable, Queryable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "documentos"]
pub struct NovoDocumento {
    pub tipo_de_documento: TipoDocumento,
    pub valor: BigDecimal,
    pub data: chrono::NaiveDate,
    pub descricao: String,
    pub id_usuario: Uuid,
}

#[derive(Debug, Insertable, Queryable, Serialize, Deserialize, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "documentos"]
pub struct AtualizarDocumento {
    pub id: Uuid,
    pub tipo_de_documento: Option<TipoDocumento>,
    pub valor: Option<BigDecimal>,
    pub data: Option<chrono::NaiveDate>,
    pub descricao: Option<String>,
}
