ALTER TABLE
    posts
ADD
    COLUMN points INT NOT NULL DEFAULT 0,
ADD
    COLUMN views INT NOT NULL DEFAULT 0,
ADD
    COLUMN featured BOOLEAN NOT NULL DEFAULT FALSE;

CREATE TABLE IF NOT EXISTS tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    color VARCHAR(255) NOT NULL
);

INSERT INTO
    tags (name, color)
VALUES
    ('Web', 'green'),
    ('SQL Injection', 'green'),
    ('Forensics', 'green'),
    ('Reversing', 'red'),
    ('Binary Exploitation', 'red'),
    ('Scripting', 'blue'),
    ('XSS', 'blue'),
    ('SSRF', 'blue'),
    ('Encoding', 'yellow'),
    ('Crypto', 'yellow'),
    ('XXE', 'yellow'),
    ('LFI', 'yellow'),
    ('RCE', 'red'),
    ('Miscellaneous', 'yellow'),
    ('Filter Bypass', 'red'),
    ('Hardware', 'red'),
    ('Linux', 'yellow'),
    ('Mobile', 'green'),
    ('Game Hacking', 'yellow'),
    ('OSINT', 'blue');

CREATE TABLE IF NOT EXISTS post_tags (
    post_id INT NOT NULL REFERENCES posts (id) ON DELETE CASCADE,
    tag_id INT NOT NULL REFERENCES tags (id) ON DELETE CASCADE,
    PRIMARY KEY (post_id, tag_id)
);