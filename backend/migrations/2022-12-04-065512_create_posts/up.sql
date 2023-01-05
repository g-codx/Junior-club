CREATE TABLE posts
(
    id          SERIAL PRIMARY KEY,
    user_id     INT NOT NULL,
    title       VARCHAR NOT NULL,
    preview     TEXT    NOT NULL,
    body        TEXT    NOT NULL,
    published   BOOLEAN NOT NULL DEFAULT FALSE,
    public_date TIMESTAMP        DEFAULT NULL,
    search_tag  VARCHAR NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY (user_id)
            REFERENCES users (id)
);