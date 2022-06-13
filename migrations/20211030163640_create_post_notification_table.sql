-- Add migration script here
CREATE TABLE IF NOT EXISTS post_notifications (
    notification_id SERIAL NOT NULL PRIMARY KEY,
    notification_type VARCHAR(50) NOT NULL,
    user_id INTEGER NOT NULL,
    post_id INTEGER NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (post_id) REFERENCES posts(post_id) ON DELETE CASCADE
);  