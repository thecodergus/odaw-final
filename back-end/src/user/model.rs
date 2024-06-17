use chrono::{Duration, NaiveDate};
use postgres::Error;

use crate::db::get_client;
pub struct User {
    id: Option<String>,
    nome: String,
    email: Option<String>,
    data_de_nascimento: NaiveDate,
    senha: Option<String>,
}

pub fn find_user(email: String, password: String) -> Result<User, Error> {
    match get_client().query_one(
        "SELECT id, nome, data_de_nascimento FROM usuario WHERE email=$1 AND senha=$2 ",
        &[&email, &password],
    ) {
        Ok(user) => Ok(User {
            id: Some(user.get(0)),
            nome: user.get(1),
            email: Some(email.to_owned()),
            senha: None,
            data_de_nascimento: NaiveDate::parse_from_str(user.get(2), "%Y-%m-%d").unwrap(),
        }),
        Err(err) => Err(err),
    }
}

pub fn register_user(new_user: User) -> Result<(), Error> {
    match get_client().execute(
        "INSERT INTO usuario (nome, email, senha, data_de_nascimento) VALUES ($1, $2, $3, $4)",
        &[
            &new_user.nome,
            &new_user.email,
            &new_user.senha,
            &new_user.data_de_nascimento.format("%Y-%m-%d").to_string(),
        ],
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub fn update_user(user: User) -> Result<(), Error> {
    match get_client().execute(
        "UPDATE usuario SET nome = $1, email = $2, senha = $3, data_de_nascimento = $4 WHERE id = $5",
        &[
            &user.nome,
            &user.email,
            &user.senha,
            &user.data_de_nascimento.format("%Y-%m-%d").to_string(),
            &user.id,
        ],
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
