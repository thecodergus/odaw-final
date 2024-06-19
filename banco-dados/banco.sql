-- Instalando UUID
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Criando enumerador
CREATE TYPE DOCUMENTO_TIPO AS ENUM ('conta', 'receita');

-- Criando tabela usuario
CREATE TABLE IF NOT EXISTS usuario (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nome TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    senha TEXT NOT NULL,
    data_de_nascimento DATE NOT NULL
);

-- Criando tabela documento
CREATE TABLE IF NOT EXISTS documento (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tipo_de_documento DOCUMENTO_TIPO NOT NULL,
    valor DECIMAL(12, 2) NOT NULL,
    data DATE NOT NULL,
    descricao TEXT,
    id_usuario UUID REFERENCES usuario(id)
);

-- Criando tabela categoria
CREATE TABLE IF NOT EXISTS categoria (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nome TEXT UNIQUE NOT NULL,
    id_documento UUID REFERENCES documento(id)
);
