CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    password_hash VARCHAR(255) NOT NULL
);

-- Default password is 'secret'
INSERT INTO
    users (password_hash)
VALUES
    (
        '$2a$12$hcRN2A.SjKBNZ9rrQW.VJ.z0gZmxbT8wZ7YtgxX19.3tDY3kclwra'
    );