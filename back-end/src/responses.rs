use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}
