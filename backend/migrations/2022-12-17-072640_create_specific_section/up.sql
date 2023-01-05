CREATE TABLE links
(
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    link VARCHAR NOT NULL,
    link_type VARCHAR NOT NULL,
    section_type VARCHAR NOT NULL
);