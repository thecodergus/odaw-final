use super::model::*;
use crate::responses::GenericResponse;
use std::panic;

use rocket::response::status;
use rocket::response::Redirect;
use rocket::{
    delete, get, http::Status, patch, post, put, response::status::Custom, serde::json::Json, State,
};
use serde_json::json;

#[post("/", data = "<user_input>", format = "json")]
pub async fn create_user(
    user_input: Json<User>,
) -> Result<Json<GenericResponse>, Json<GenericResponse>> {
    let new_user = User {
        id: None,
        nome: user_input.nome.clone(),
        email: user_input.email.clone(),
        data_de_nascimento: user_input.data_de_nascimento.clone(),
        senha: user_input.senha.clone(),
    };

    match register_user(new_user).await {
        Ok(_) => {
            let generic_response = GenericResponse {
                status: "success".to_string(),
                message: "User created".to_string(),
            };
            Ok(Json(generic_response))
        }
        Err(err) => {
            let generic_response = GenericResponse {
                status: "error".to_string(),
                message: err.to_string(),
            };
            Err(Json(generic_response))
        }
    }
}
