-- Add migration script here
CREATE TABLE IF NOT EXISTS reports (
    report_id SERIAL NOT NULL PRIMARY KEY,
    post_id INTEGER,
    comment_id INTEGER,
    reason TEXT NOT NULL,
    addressed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (post_id) REFERENCES posts(post_id) ON DELETE CASCADE,
    FOREIGN KEY (comment_id) REFERENCES comments(comment_id) ON DELETE CASCADE
);