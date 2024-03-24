-- Add migration script here
DROP TABLE IF EXISTS urls;
CREATE TABLE urls (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    orignal TEXT NOT NULL,
    shorten TEXT NOT NULL
);
