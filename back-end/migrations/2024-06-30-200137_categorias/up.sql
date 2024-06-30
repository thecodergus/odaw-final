-- Your SQL goes here
-- Criando tabela categoria
CREATE TABLE IF NOT EXISTS categorias (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nome TEXT UNIQUE NOT NULL,
    id_documento UUID REFERENCES documentos(id)
);
