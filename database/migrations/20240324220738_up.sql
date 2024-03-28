-- Add migration script here
DROP TABLE IF EXISTS urls;
CREATE TABLE urls (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    url TEXT NOT NULL,
    hash TEXT NOT NULL
);
