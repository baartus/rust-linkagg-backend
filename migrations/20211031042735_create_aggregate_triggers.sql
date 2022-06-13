-- create user
CREATE OR REPLACE FUNCTION create_user_aggregates()
RETURNS TRIGGER AS $agg_create$
BEGIN
INSERT INTO user_aggregates (user_id)
VALUES (new.user_id);
RETURN NEW;
END;
$agg_create$ LANGUAGE plpgsql;
CREATE TRIGGER user_agg_create AFTER INSERT ON users FOR EACH ROW EXECUTE PROCEDURE create_user_aggregates();

-- create comment
CREATE OR REPLACE FUNCTION create_comment_aggregates()
RETURNS TRIGGER AS $agg_create$
BEGIN
INSERT INTO comment_aggregates (comment_id)
VALUES (new.comment_id);
UPDATE post_aggregates
SET replies = (replies + 1)
WHERE post_id = new.post_id;
UPDATE user_aggregates
SET number_of_comments = (number_of_comments + 1)
WHERE user_id = new.user_id;
RETURN NEW;
END;
$agg_create$ LANGUAGE plpgsql;

CREATE TRIGGER comment_agg_create AFTER INSERT ON comments FOR EACH ROW EXECUTE PROCEDURE create_comment_aggregates();

-- delete comment
CREATE OR REPLACE FUNCTION comment_delete_aggregates()
RETURNS TRIGGER AS $agg_create$
BEGIN
IF old.post_id IS NOT NULL THEN
    UPDATE post_aggregates
    SET replies = (replies - 1)
    WHERE post_id = old.post_id;
END IF;
UPDATE user_aggregates
SET number_of_comments = (number_of_comments - 1)
WHERE user_id = old.user_id;
RETURN OLD;
END;
$agg_create$ LANGUAGE plpgsql;

CREATE TRIGGER comment_agg_delete BEFORE DELETE ON comments FOR EACH ROW EXECUTE PROCEDURE comment_delete_aggregates();

-- create post
CREATE OR REPLACE FUNCTION create_post_aggregates()
RETURNS TRIGGER AS $agg_create$
BEGIN
INSERT INTO post_aggregates (post_id)
VALUES (new.post_id);
UPDATE guild_aggregates
SET number_of_posts = (number_of_posts + 1)
WHERE guild_tag = new.guild_tag;
UPDATE user_aggregates
SET number_of_posts = (number_of_posts + 1)
WHERE user_id = new.user_id;
RETURN NEW;
END;
$agg_create$ LANGUAGE plpgsql;

CREATE TRIGGER post_agg_create AFTER INSERT ON posts FOR EACH ROW EXECUTE PROCEDURE create_post_aggregates();

-- delete post
CREATE OR REPLACE FUNCTION post_delete_aggregates()
RETURNS TRIGGER AS $agg_create$
BEGIN
IF old.guild_tag IS NOT NULL THEN
    UPDATE guild_aggregates
    SET number_of_posts = (number_of_posts - 1)
    WHERE guild_tag = old.guild_tag;
END IF;
UPDATE user_aggregates
SET number_of_posts = (number_of_posts - 1)
WHERE user_id = old.user_id;
RETURN OLD;
END;
$agg_create$ LANGUAGE plpgsql;

CREATE TRIGGER post_agg_delete BEFORE DELETE ON posts FOR EACH ROW EXECUTE PROCEDURE post_delete_aggregates();

-- create guild
CREATE OR REPLACE FUNCTION create_guild_aggregates()
RETURNS TRIGGER AS $agg_create$
BEGIN
INSERT INTO guild_aggregates (guild_tag)
VALUES (new.guild_tag);
RETURN NEW;
END;
$agg_create$ LANGUAGE plpgsql;

CREATE TRIGGER guild_agg_create AFTER INSERT ON guilds FOR EACH ROW EXECUTE PROCEDURE create_guild_aggregates();

-- join guild
CREATE OR REPLACE FUNCTION guild_agg_new_member()
RETURNS TRIGGER AS $agg_create$
BEGIN
UPDATE guild_aggregates
SET members = (members + 1)
WHERE guild_tag = new.guild_tag;
UPDATE user_aggregates
SET number_of_memberships = (number_of_memberships + 1)
WHERE user_id = new.user_id;
RETURN NEW;
END;
$agg_create$ LANGUAGE plpgsql;

CREATE TRIGGER guild_agg_member_join AFTER INSERT ON guild_memberships FOR EACH ROW EXECUTE PROCEDURE guild_agg_new_member();

-- leave guild
CREATE OR REPLACE FUNCTION guild_agg_member_leaves()
RETURNS TRIGGER AS $agg_create$
BEGIN
UPDATE guild_aggregates
SET members = (members - 1)
WHERE guild_tag = old.guild_tag;
UPDATE user_aggregates
SET number_of_memberships = (number_of_memberships - 1)
WHERE user_id = old.user_id;
RETURN OLD;
END;
$agg_create$ LANGUAGE plpgsql;

CREATE TRIGGER guild_agg_member_left BEFORE DELETE ON guild_memberships FOR EACH ROW EXECUTE PROCEDURE guild_agg_member_leaves();

-- vote post
CREATE OR REPLACE FUNCTION post_agg_new_vote()
RETURNS TRIGGER AS $agg_create$
BEGIN
IF new.up = TRUE THEN
    UPDATE post_aggregates
    SET upvotes = (upvotes + 1)
    WHERE post_id = new.post_id;

    UPDATE user_aggregates
    SET upvotes = (upvotes + 1)
    WHERE user_id = (SELECT user_id FROM posts where posts.post_id = new.post_id);
ELSE
    UPDATE post_aggregates
    SET downvotes = (downvotes + 1)
    WHERE post_id = new.post_id;

    UPDATE user_aggregates
    SET downvotes = (downvotes + 1)
    WHERE user_id = (SELECT user_id FROM posts where posts.post_id = new.post_id);
END IF;

RETURN NEW;
END;
$agg_create$ LANGUAGE plpgsql;
CREATE TRIGGER post_new_vote_aggs AFTER INSERT ON post_votes FOR EACH ROW EXECUTE PROCEDURE post_agg_new_vote();

--update post vote
CREATE OR REPLACE FUNCTION post_agg_changed_vote()
RETURNS TRIGGER AS $agg_create$
BEGIN
IF new.up = TRUE THEN
    UPDATE post_aggregates
    SET upvotes = (upvotes + 1), downvotes = (downvotes - 1)
    WHERE post_id = new.post_id;

    UPDATE user_aggregates
    SET upvotes = (upvotes + 1), downvotes = (downvotes - 1)
    WHERE user_id = (SELECT user_id FROM posts where posts.post_id = new.post_id);
ELSE
    UPDATE post_aggregates
    SET downvotes = (downvotes + 1), upvotes = (upvotes -1)
    WHERE post_id = new.post_id;

    UPDATE user_aggregates
    SET upvotes = (upvotes - 1), downvotes = (downvotes + 1)
    WHERE user_id = (SELECT user_id FROM posts where posts.post_id = new.post_id);
END IF;
RETURN NEW;
END;
$agg_create$ LANGUAGE plpgsql;
CREATE TRIGGER post_vote_update_aggs AFTER UPDATE ON post_votes FOR EACH ROW EXECUTE PROCEDURE post_agg_changed_vote();

--delete post vote
CREATE OR REPLACE FUNCTION post_agg_deleted_vote()
RETURNS TRIGGER AS $agg_create$
BEGIN
IF old.up = TRUE THEN
    UPDATE post_aggregates
    SET upvotes = (upvotes - 1)
    WHERE post_id = old.post_id;

    UPDATE user_aggregates
    SET upvotes = (upvotes - 1)
    WHERE user_id = (SELECT user_id FROM posts where posts.post_id = old.post_id);
ELSE
    UPDATE post_aggregates
    SET downvotes = (downvotes - 1)
    WHERE post_id = old.post_id;

    UPDATE user_aggregates
    SET downvotes = (downvotes - 1)
    WHERE user_id = (SELECT user_id FROM posts where posts.post_id = old.post_id);
END IF;
RETURN OLD;
END;
$agg_create$ LANGUAGE plpgsql;
CREATE TRIGGER post_vote_delete_aggs BEFORE DELETE ON post_votes FOR EACH ROW EXECUTE PROCEDURE post_agg_deleted_vote();

--update comment vote
CREATE OR REPLACE FUNCTION comment_agg_changed_vote()
RETURNS TRIGGER AS $agg_create$
BEGIN
IF new.up = TRUE THEN
    UPDATE comment_aggregates
    SET upvotes = (upvotes + 1), downvotes = (downvotes - 1)
    WHERE comment_id = new.comment_id;

    UPDATE user_aggregates
    SET upvotes = (upvotes + 1), downvotes = (downvotes - 1)
    WHERE user_id = (SELECT user_id FROM comments where comments.comment_id = new.comment_id);
ELSE
    UPDATE comment_aggregates
    SET downvotes = (downvotes + 1), upvotes = (upvotes -1)
    WHERE comment_id = new.comment_id;

    UPDATE user_aggregates
    SET upvotes = (upvotes - 1), downvotes = (downvotes + 1)
    WHERE user_id = (SELECT user_id FROM comments where comments.comment_id = new.comment_id);
END IF;
RETURN NEW;
END;
$agg_create$ LANGUAGE plpgsql;
CREATE TRIGGER comment_vote_update_aggs AFTER UPDATE ON comment_votes FOR EACH ROW EXECUTE PROCEDURE comment_agg_changed_vote();

--delete comment vote
CREATE OR REPLACE FUNCTION comment_agg_deleted_vote()
RETURNS TRIGGER AS $agg_create$
BEGIN
IF old.up = TRUE THEN
    UPDATE comment_aggregates
    SET upvotes = (upvotes - 1)
    WHERE comment_id = old.comment_id;

    UPDATE user_aggregates
    SET upvotes = (upvotes - 1)
    WHERE user_id = (SELECT user_id FROM comments where comments.comment_id = old.comment_id);
ELSE
    UPDATE comment_aggregates
    SET downvotes = (downvotes - 1)
    WHERE comment_id = old.comment_id;

    UPDATE user_aggregates
    SET downvotes = (downvotes - 1)
    WHERE user_id = (SELECT user_id FROM comments where comments.comment_id = old.comment_id);
END IF;
RETURN OLD;
END;
$agg_create$ LANGUAGE plpgsql;
CREATE TRIGGER comment_vote_delete_aggs BEFORE DELETE ON comment_votes FOR EACH ROW EXECUTE PROCEDURE comment_agg_deleted_vote();


-- vote comment
CREATE OR REPLACE FUNCTION comment_agg_new_vote()
RETURNS TRIGGER AS $agg_create$
BEGIN
IF new.up = TRUE THEN
    UPDATE comment_aggregates
    SET upvotes = (upvotes + 1)
    WHERE comment_id = new.comment_id;

    UPDATE user_aggregates
    SET upvotes = (upvotes + 1)
    WHERE user_id = (SELECT user_id FROM comments where comments.comment_id = new.comment_id);
ELSE
    UPDATE comment_aggregates
    SET downvotes = (downvotes + 1)
    WHERE comment_id = new.comment_id;

    UPDATE user_aggregates
    SET downvotes = (downvotes + 1)
    WHERE user_id = (SELECT user_id FROM comments where comments.comment_id = new.comment_id);
END IF;

RETURN NEW;
END;
$agg_create$ LANGUAGE plpgsql;
CREATE TRIGGER comment_new_vote_aggs AFTER INSERT ON comment_votes FOR EACH ROW EXECUTE PROCEDURE comment_agg_new_vote();