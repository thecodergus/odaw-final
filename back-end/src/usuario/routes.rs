use rocket_contrib::templates::Template;
use std::collections::HashMap;

use diesel::prelude::*;

use crate::schema::*;

use crate::usuario::model::{NovoUsuario, Usuario};

use rocket::http::ContentType;
use rocket::Data;

use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
