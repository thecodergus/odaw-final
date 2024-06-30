// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "documento_tipo"))]
    pub struct DocumentoTipo;
}

diesel::table! {
    categorias (id) {
        id -> Uuid,
        nome -> Text,
        id_documento -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::DocumentoTipo;

    documentos (id) {
        id -> Uuid,
        tipo_de_documento -> DocumentoTipo,
        valor -> Numeric,
        data -> Date,
        descricao -> Nullable<Text>,
        id_usuario -> Nullable<Uuid>,
    }
}

diesel::table! {
    usuarios (id) {
        id -> Uuid,
        nome -> Text,
        email -> Text,
        senha -> Text,
        data_de_nascimento -> Date,
    }
}

diesel::joinable!(categorias -> documentos (id_documento));
diesel::joinable!(documentos -> usuarios (id_usuario));

diesel::allow_tables_to_appear_in_same_query!(
    categorias,
    documentos,
    usuarios,
);
