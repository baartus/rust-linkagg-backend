-- Add migration script here
CREATE TABLE IF NOT EXISTS password_resets (
    reset_hash VARCHAR(255) NOT NULL UNIQUE PRIMARY KEY,
    user_id INTEGER NOT NULL,
    verified_email BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);