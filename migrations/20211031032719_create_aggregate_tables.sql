-- Add migration script here
CREATE TABLE IF NOT EXISTS post_aggregates (
    post_id INTEGER NOT NULL,
    upvotes INTEGER NOT NULL DEFAULT 0,
    downvotes INTEGER NOT NULL DEFAULT 0,
    replies INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (post_id) REFERENCES posts(post_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS comment_aggregates (
    comment_id INTEGER NOT NULL,
    upvotes INTEGER NOT NULL DEFAULT 0,
    downvotes INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (comment_id) REFERENCES comments(comment_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS guild_aggregates (
    guild_tag VARCHAR(20) NOT NULL,
    members INTEGER NOT NULL DEFAULT 0,
    number_of_posts INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (guild_tag) REFERENCES guilds(guild_tag) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_aggregates (
    user_id INTEGER NOT NULL,
    upvotes INTEGER NOT NULL DEFAULT 0,
    downvotes INTEGER NOT NULL DEFAULT 0,
    number_of_posts INTEGER NOT NULL DEFAULT 0,
    number_of_comments INTEGER NOT NULL DEFAULT 0,
    number_of_memberships INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);