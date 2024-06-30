use diesel::{pg::PgConnection, Connection};
use dotenvy::dotenv;
use std::env;

pub fn estabelecer_conexao() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL deve ser setada");
    PgConnection::establish(&database_url).expect(&format!(
        "Erro ao tentar conectar ao Postgres: {}",
        database_url
    ))
}
