-- Add migration script here
CREATE TABLE IF NOT EXISTS comment_notifications (
    notification_id SERIAL NOT NULL PRIMARY KEY,
    notification_type VARCHAR(50) NOT NULL,
    user_id INTEGER NOT NULL,
    comment_id INTEGER NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (comment_id) REFERENCES comments(comment_id) ON DELETE CASCADE
);  