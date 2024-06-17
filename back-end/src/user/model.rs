use chrono::{Duration, NaiveDate};

pub struct User {
    id: String,
    nome: String,
    email: String,
    data_de_nascimento: NaiveDate,
}
