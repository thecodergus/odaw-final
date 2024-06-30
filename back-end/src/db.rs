use postgres::{Client, Error, NoTls};

use dotenvy::dotenv;
use std::env;

pub fn get_client() -> Result<Client, Error> {
    dotenv().ok();

    let user_db = env::var("POSTGRES_USER").expect("Env com usuario do postgres não encontrado");
    let password_db =
        env::var("POSTGRES_PASSWORD").expect("Env com senha do postgres não encontrado");
    let host_db = env::var("POSTGRES_HOST").expect("Env com host do postgres não encontrado");
    let port_db = env::var("POSTGRES_PORT").expect("Env com porta do postgres não encontrado");
    let database_db =
        env::var("POSTGRES_DATABASE").expect("Env com database do postgres não encontrado");

    let database_url: String = format!(
        "postgresql://{}:{}@{}:{}/{}",
        user_db, password_db, host_db, port_db, database_db
    );

    match Client::connect(&database_url, NoTls) {
        Ok(valor) => Ok(valor),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use postgres::{Client, NoTls};

    use super::*;

    #[test]
    fn test_get_client() {
        let client = get_client();
        assert!(!client.unwrap().is_closed());
    }
}
