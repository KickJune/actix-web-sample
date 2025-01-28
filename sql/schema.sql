DROP TABLE IF EXISTS items;

CREATE TABLE items
(
    id           SERIAL PRIMARY KEY,
    name         VARCHAR(50) NOT NULL,
    price        INTEGER     NOT NULL,
    release_date DATE,
    description  VARCHAR(200)
);
