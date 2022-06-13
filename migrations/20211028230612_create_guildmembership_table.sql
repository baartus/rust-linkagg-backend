-- Add migration script here
CREATE TABLE IF NOT EXISTS guild_memberships (
    membership_id SERIAL NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    guild_tag VARCHAR(20) NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    is_moderator BOOLEAN NOT NULL DEFAULT FALSE,
    is_banned BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (guild_tag) REFERENCES guilds(guild_tag) ON DELETE CASCADE
);