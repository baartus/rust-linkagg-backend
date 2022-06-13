-- Add migration script here
CREATE TABLE IF NOT EXISTS guilds (
    guild_tag VARCHAR(20) NOT NULL UNIQUE PRIMARY KEY,
    guild_name VARCHAR(50) NOT NULL,
    guild_description TEXT,
    avatar_url VARCHAR(255),
    banner_url VARCHAR(255),
    is_banned BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

