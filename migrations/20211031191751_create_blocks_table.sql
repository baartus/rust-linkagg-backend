-- Add migration script here
CREATE TABLE IF NOT EXISTS blocks (
    user_id INTEGER NOT NULL,
    blocked_user_username VARCHAR(20) NOT NULL,
    PRIMARY KEY(user_id, blocked_user_username),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (blocked_user_username) REFERENCES users(username) ON DELETE CASCADE
);