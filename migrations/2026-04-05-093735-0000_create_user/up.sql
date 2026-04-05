CREATE TABLE users (
    id         SERIAL PRIMARY KEY,
    username   VARCHAR NOT NULL,
    email      VARCHAR NOT NULL,
    full_name  VARCHAR,
    active     BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
