-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    user_id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    status BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS messages (
    id INT AUTO_INCREMENT PRIMARY KEY,
    sender INT NOT NULL,
    receiver INT NOT NULL,
    message_type VARCHAR(50) NOT NULL,
    content TEXT NOT NULL,
    timestamp DATETIME NOT NULL,
    dead_time DATETIME NOT NULL
);