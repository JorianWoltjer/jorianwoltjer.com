CREATE TABLE IF NOT EXISTS links (
    id SERIAL PRIMARY KEY,
    folder INT NOT NULL REFERENCES folders(id) ON DELETE CASCADE,
    url TEXT NOT NULL UNIQUE,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    img VARCHAR(255) NOT NULL,
    featured BOOLEAN NOT NULL DEFAULT FALSE,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);