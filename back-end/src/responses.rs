use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RespontaGenerica {
    pub status: String,
    pub codigo: i32,
    pub mensagem: Option<String>,
}
