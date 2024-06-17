use chrono::{Duration, NaiveDate};
use rust_decimal::Decimal;

pub enum Type {
    Bill,
    Revenue,
}

pub struct Document {
    id: String,
    tipo_documento: Type,
    valor: Decimal,
    data: NaiveDate,
    descricao: String,
    id_usuario: String,
}
