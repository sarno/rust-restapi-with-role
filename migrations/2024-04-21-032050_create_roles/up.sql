-- Your SQL goes here
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    code varchar(64) NOT NULL,
    name VARCHAR(255) NOT NULL,
    create_at TIMESTAMP DEFAULT NOW() NOT NULL
)