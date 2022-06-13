-- Add migration script here
CREATE OR REPLACE FUNCTION create_comment_notification() 
RETURNS TRIGGER AS $noti_create$
BEGIN

-- TODO: make it so notifications aren't generated if you comment on your own post or comment
IF new.parent_comment_id IS NULL THEN
    INSERT INTO comment_notifications(notification_type, user_id, comment_id)
    VALUES (
    'reply',
    (SELECT user_id FROM posts where posts.post_id = new.post_id),
    new.comment_id);
ELSE
    INSERT INTO comment_notifications(notification_type, user_id, comment_id)
    VALUES (
    'reply',
    (SELECT user_id FROM comments where comments.comment_id = new.parent_comment_id),
    new.comment_id);
END IF;

RETURN NEW;
END;
$noti_create$ LANGUAGE plpgsql;

CREATE TRIGGER comment_trig AFTER INSERT ON comments FOR EACH ROW EXECUTE PROCEDURE create_comment_notification();

--post notifications. add receives notifications bool to guild membership
--CREATE OR REPLACE FUNCTION create_post_notification()
--RETURNS TRIGGER AS $noti_create$
--BEGIN
--logic here
--RETURN NEW;
--END;
--$noti_create$ LANGUAGE plpgsql;

--CREATE TRIGGER post_trig AFTER INSERT ON posts FOR EACH ROW EXECUTE PROCEDURE create_post_notification();