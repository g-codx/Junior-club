CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    unique_id VARCHAR NOT NULL,
    role VARCHAR NOT NULL default 'user',
    UNIQUE (email),
    UNIQUE (name)
);

