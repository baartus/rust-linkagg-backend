-- Add migration script here
CREATE TABLE IF NOT EXISTS posts (
    post_id SERIAL NOT NULL PRIMARY KEY,
    guild_tag VARCHAR(20) NOT NULL,
    user_id INTEGER NOT NULL,
    image_url VARCHAR(255),
    link_url VARCHAR(255),
    title VARCHAR(255) NOT NULL DEFAULT 'Untitled Post',
    body TEXT,
    is_locked BOOLEAN NOT NULL DEFAULT FALSE,
    is_edited BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (guild_tag) REFERENCES guilds(guild_tag) ON DELETE SET NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);