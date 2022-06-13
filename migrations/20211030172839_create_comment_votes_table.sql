-- Add migration script here
CREATE TABLE IF NOT EXISTS comment_votes (
    comment_id SERIAL NOT NULL,
    user_id SERIAL NOT NULL,
    up BOOLEAN NOT NULL,
    PRIMARY KEY(comment_id, user_id),
    FOREIGN KEY (comment_id) REFERENCES comments(comment_id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);