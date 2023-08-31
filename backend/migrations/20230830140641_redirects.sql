CREATE TABLE post_redirects (
    id INT AUTO_INCREMENT PRIMARY KEY,
    slug VARCHAR(255) NOT NULL UNIQUE,
    post_id INT NOT NULL,
    FOREIGN KEY (post_id) REFERENCES posts(id)
);

CREATE TABLE folder_redirects (
    id INT AUTO_INCREMENT PRIMARY KEY,
    slug VARCHAR(255) NOT NULL UNIQUE,
    folder_id INT NOT NULL,
    FOREIGN KEY (folder_id) REFERENCES folders(id)
)