use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RespontaGenerica {
    pub status: String,
    pub mensagem: Option<String>,
}
