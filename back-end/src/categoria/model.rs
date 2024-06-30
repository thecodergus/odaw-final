use crate::schema::categorias;
use bigdecimal::BigDecimal;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};
use diesel::serialize::{self, ToSql};
use diesel_derive_enum::DbEnum;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[serde(crate = "rocket::serde")]
#[diesel(check_for_backend(Pg))]
pub struct Categoria {
    pub id: Uuid,
    pub nome: String,
    pub id_documento: Option<Uuid>,
}

#[derive(Debug, Insertable, Serialize, Deserialize, Queryable, Selectable)]
#[serde(crate = "rocket::serde")]
#[table_name = "categorias"]
pub struct NovaCategoria {
    pub nome: String,
    pub id_documento: Uuid,
}
