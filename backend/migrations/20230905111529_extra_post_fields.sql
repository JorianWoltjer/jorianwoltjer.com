ALTER TABLE
    posts
ADD
    COLUMN points INTEGER NOT NULL DEFAULT 0,
ADD
    COLUMN views INTEGER NOT NULL DEFAULT 0,
ADD
    COLUMN featured BOOLEAN NOT NULL DEFAULT FALSE;

CREATE TABLE IF NOT EXISTS tags (
    id INT AUTO_INCREMENT PRIMARY KEY,
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
    post_id INT NOT NULL,
    tag_id INT NOT NULL,
    PRIMARY KEY (post_id, tag_id),
    FOREIGN KEY (post_id) REFERENCES posts (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
);