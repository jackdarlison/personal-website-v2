CREATE TABLE IF NOT EXISTS posts (
    id SERIAL PRIMARY KEY,
    date DATE NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    tags TEXT[]
);