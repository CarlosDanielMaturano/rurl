-- Add migration script here
DROP TABLE IF EXISTS urls;
CREATE TABLE urls (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    original TEXT NOT NULL,
    shorten TEXT NOT NULL
);
