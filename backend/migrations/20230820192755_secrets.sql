CREATE EXTENSION pgcrypto;

CREATE TABLE IF NOT EXISTS secrets (
    name VARCHAR(255) NOT NULL PRIMARY KEY,
    value VARCHAR(255) NOT NULL
);

-- Default password is 'secret'
INSERT INTO
    secrets
VALUES
    (
        'password_hash',
        '$2a$12$hcRN2A.SjKBNZ9rrQW.VJ.z0gZmxbT8wZ7YtgxX19.3tDY3kclwra'
    );