-- Add migration script here
CREATE TABLE IF NOT EXISTS comments (
    comment_id SERIAL NOT NULL PRIMARY KEY,
    post_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    parent_comment_id INTEGER,
    body TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_edited BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (post_id) REFERENCES posts(post_id),--i think, technically, this should be on delete set null... but I don't think necessary.
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (parent_comment_id) REFERENCES comments(comment_id) ON DELETE SET NULL
);