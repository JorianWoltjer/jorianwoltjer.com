INSERT INTO
    secrets
VALUES
    ('hmac_key', encode(gen_random_bytes(32), 'hex'));

ALTER TABLE
    posts
ADD
    COLUMN hidden BOOLEAN NOT NULL;