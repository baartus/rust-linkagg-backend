-- Add migration script here
CREATE OR REPLACE VIEW detailed_post_view AS
SELECT posts.post_id, posts.guild_tag, posts.image_url, posts.link_url, posts.title, posts.body, posts.is_locked, posts.is_edited, posts.created_at, users.username, users.avatar_url, users.is_admin, users.is_verified, post_aggregates.upvotes, post_aggregates.downvotes, post_aggregates.replies
FROM ((posts INNER JOIN users ON posts.user_id = users.user_id) INNER JOIN post_aggregates ON posts.post_id = post_aggregates.post_id)
ORDER BY created_at DESC;

CREATE OR REPLACE VIEW detailed_comment_view AS
SELECT comments.comment_id, comments.post_id, comments.parent_comment_id, comments.body, comments.created_at, comments.is_edited, users.username, users.avatar_url, users.is_admin, users.is_verified, comment_aggregates.upvotes, comment_aggregates.downvotes
FROM ((comments INNER JOIN users ON comments.user_id = users.user_id) INNER JOIN comment_aggregates ON comment_aggregates.comment_id = comments.comment_id)
ORDER BY created_at DESC;

CREATE OR REPLACE VIEW detailed_guild_view AS
SELECT guilds.*, guild_aggregates.members, guild_aggregates.number_of_posts
FROM (guilds INNER JOIN guild_aggregates ON guild_aggregates.guild_tag = guilds.guild_tag)
ORDER BY members;

CREATE OR REPLACE VIEW short_guild_view AS
SELECT guilds.guild_tag, guilds.guild_name, guilds.avatar_url, guild_aggregates.members, guild_aggregates.number_of_posts
FROM (guilds INNER JOIN guild_aggregates ON guild_aggregates.guild_tag = guilds.guild_tag)
ORDER BY members;

CREATE OR REPLACE VIEW detailed_user_view AS
SELECT users.username, users.avatar_url, users.is_admin, users.is_verified, users.is_banned, users.created_at, user_aggregates.upvotes, user_aggregates.downvotes, user_aggregates.number_of_posts, user_aggregates.number_of_comments, user_aggregates.number_of_memberships
FROM (users INNER JOIN user_aggregates ON users.user_id = user_aggregates.user_id);

CREATE OR REPLACE VIEW user_personal_view AS
SELECT users.username, users.email, users.avatar_url
FROM users;

--seed admin account, the best I can do right now is also seed a password reset for it that you can update upon accessing
INSERT INTO users (email, username, password_hash, is_admin) VALUES ('email@email.com', 'admin', 'password', TRUE);

--seed guild that admin is member of, test aggregates
INSERT INTO guilds (guild_tag, guild_name) VALUES ('testguild', 'Test Guild');
INSERT INTO guild_memberships (user_id, guild_tag, is_admin) VALUES (1, 'testguild', TRUE);


--seeded password reset for admin account
INSERT INTO password_resets (reset_hash, user_id, verified_email) VALUES ('admin', 1, TRUE);