CREATE TABLE IF NOT EXISTS folders (
    id INT AUTO_INCREMENT PRIMARY KEY,
    parent INT DEFAULT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    img VARCHAR(255) NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (parent) REFERENCES folders(id) ON DELETE CASCADE
);

ALTER TABLE
    posts
ADD
    COLUMN folder INT NOT NULL,
ADD
    FOREIGN KEY (folder) REFERENCES folders(id) ON DELETE CASCADE;