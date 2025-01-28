-- Add up migration script here
CREATE TABLE items
(
    id           SERIAL PRIMARY KEY,
    name         VARCHAR(50) NOT NULL,
    price        INTEGER     NOT NULL,
    description  VARCHAR(200)
);