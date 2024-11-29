CREATE TABLE IF NOT EXISTS posts (
    id SERIAL PRIMARY KEY,
    date DATE NOT NULL DEFAULT CURRENT_DATE,
    title TEXT,
    body TEXT
);