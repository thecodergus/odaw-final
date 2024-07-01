-- Your SQL goes here
-- Criando tabela documento
CREATE TABLE IF NOT EXISTS documentos (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tipo_de_documento DOCUMENTO_TIPO NOT NULL,
    valor DECIMAL(12, 2) NOT NULL,
    data DATE NOT NULL,
    descricao TEXT,
    id_usuario UUID REFERENCES usuarios(id) NOT NULL
);
