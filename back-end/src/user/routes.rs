use super::model::*;

use rocket::response::status;
use rocket::response::Redirect;
use rocket::{delete, get, http::Status, post, put, State};
use rocket_contrib::json::{Json, JsonValue};
use serde_json::json;

#[post("/", format = "json", data = "<novo_usuario>")]
pub fn create_user(novo_usuario: Json<User>) -> Json<JsonValue> {
    match register_user(User {
        id: None,
        nome: novo_usuario.nome.clone(),
        email: Some(
            novo_usuario
                .email
                .clone()
                .expect("E-mail é campo obrigatorio"),
        ),
        data_de_nascimento: novo_usuario.data_de_nascimento.clone(),
        senha: Some(
            novo_usuario
                .senha
                .clone()
                .expect("Senha é campo obrigatorio"),
        ),
    }) {
        Ok(_) => Json(JsonValue::from(json!({
            "status": "sucess",
            "message": "Usuario criado com sucesso"
        }))),
        Err(err) => Json(JsonValue::from(json!({
            "status": "failed",
            "message": format!("{}", err)
        }))),
    }
}
