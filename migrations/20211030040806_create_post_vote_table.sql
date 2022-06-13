-- Add migration script here
CREATE TABLE IF NOT EXISTS post_votes (
    post_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    up BOOLEAN NOT NULL,
    PRIMARY KEY(post_id, user_id),
    FOREIGN KEY (post_id) REFERENCES posts(post_id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);