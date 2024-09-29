CREATE TABLE telefones(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    numero VARCHAR(15) NOT NULL UNIQUE,
    tipo_denuncia VARCHAR(255) DEFAULT 'Spam Whatsapp',
    quantidade_denuncias INTEGER DEFAULT 1,
    data_denuncia timestamptz NOT NULL
);
