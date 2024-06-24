use chrono::{Duration, NaiveDate};
use postgres::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::get_client;

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Uuid>,
    pub nome: String,
    pub email: Option<String>,
    pub data_de_nascimento: NaiveDate,
    pub senha: Option<String>,
}

pub fn find_user(email: String, password: String) -> Result<User, Error> {
    match get_client().query_one(
        "SELECT id, nome, data_de_nascimento FROM usuario WHERE email = $1 AND senha = $2 ",
        &[&email, &password],
    ) {
        Ok(user) => Ok(User {
            id: Some(user.get("id")),
            nome: user.get("nome"),
            email: Some(email.to_owned()),
            senha: None,
            data_de_nascimento: user.get("data_de_nascimento"),
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
            &new_user.data_de_nascimento,
        ],
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub fn update_user(user: User) -> Result<(), Error> {
    match get_client().execute(
        "UPDATE usuario SET nome = $1, email = $2, data_de_nascimento = $3 WHERE id = $4",
        &[&user.nome, &user.email, &user.data_de_nascimento, &user.id],
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_insert_user() {
        let email = format!(
            "gustavo.michels.de.camargo{}@gmail.com",
            rand::thread_rng().gen_range(0..1000)
        );
        let user = User {
            email: Some(email),
            id: None,
            data_de_nascimento: NaiveDate::parse_from_str("1998-05-18", "%Y-%m-%d").unwrap(),
            nome: "Gustavo Michels de Camargo".to_string(),
            senha: Some("Pau grosso 2024".to_string()),
        };

        match register_user(user) {
            Ok(_) => println!("Tudo ok"),
            Err(err) => panic!("Deu merda: {}", err),
        };
    }

    #[test]
    fn test_find_user() {
        let rand = rand::thread_rng().gen_range(0..1000);
        let email = format!("test{}@example.com", rand).to_string();
        let password = "password123".to_string();
        let user = User {
            email: Some(email.clone()),
            id: None,
            data_de_nascimento: NaiveDate::parse_from_str("1998-05-18", "%Y-%m-%d").unwrap(),
            nome: "Gustavo Michels de Camargo".to_string(),
            senha: Some(password.clone()),
        };

        let _ = register_user(user);

        match find_user(email.clone(), password.clone()) {
            Ok(user) => {
                assert_eq!(user.email, Some(email));
                assert_eq!(user.senha, None);
            }
            Err(err) => panic!("Failed to find user: {}", err),
        };
    }

    #[test]
    fn test_update_user() {
        let rand = rand::thread_rng().gen_range(0..1000);
        let email = format!("test{}@example.com", rand).to_string();
        let user = User {
            id: None,
            nome: "John Doe".to_string(),
            email: Some(email),
            data_de_nascimento: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            senha: Some("Teste123".to_string()),
        };

        let _ = register_user(user.clone());

        let mut user_2 = find_user(user.email.unwrap(), user.senha.unwrap()).unwrap();

        user_2.nome = "Outro nome qualquer".to_string();

        match update_user(user_2) {
            Ok(_) => println!("User updated successfully"),
            Err(err) => panic!("Failed to update user: {}", err),
        };
    }
}
