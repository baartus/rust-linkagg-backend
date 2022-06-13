Current database diagram [copy and paste into mermaid.live](https://mermaid.live)
erDiagram

    Site {
        string description
    }
    User {
        int user_id
        string email
        string username
        string password_hash
        string avatar_url
        bool is_admin
        bool is_verified
        bool is_banned
        time created_at
    }
    PasswordResets {
        string reset_hash
        int user_id
        bool verified_email
    }
    UserRegistrations {
        int registration_id
        string email
        string username
        string password_hash
        string registration_hash
    }
    UserSession {
        string sessionId
        int user_id
        time created_at
    }
    Guild {
        string guild_tag
        string name
        string description
        string avatar_url
        string banner_url
        bool is_banned
        time created_at
    }
    GuildMembership {
        int membership_id
        int user_id
        string guild_tag
        bool is_admin
        bool is_moderator
        bool is_banned
    }
    Post {
        int post_id
        string guild_tag
        int user_id
        string link_url
        string title
        string body
        bool is_locked
        bool is_edited
        time created_at
    }
    Comment {
        int comment_id
        int post_id
        int user_id
        int parent_comment_id
        string body
        time timestamp
        bool is_edited
    }
    PostVote {
        int post_id
        int user_id
        bool up
    }
    CommentVote {
        int comment_id
        int user_id
        bool up
    }
    PostNotification {
        int notification_id
        string notification_type
        int user_id
        int post_id
        bool is_read
        time created_at
    }
    CommentNotification {
        int notification_id
        string notification_type
        int user_id
        int comment_id
        bool is_read
        time created_at
    }
    Report {
        int post_id
        int comment_id
        string reason
        bool addressed
    }
    Block {
        int user_id
        int blocked_user_id
    }
    Bookmark {
        int user_id
        int post_id
    }
    User ||--o{ UserSession: has_zero_or_more
    Site ||--o{ Guild: has_zero_or_more
    Guild ||--o{ User: has_zero_or_more
    User ||--o{ Post: has_zero_or_more
    User ||--o{ GuildMembership: has_zero_or_more
    User ||--o{ Comment: has_zero_or_more
    User ||--o{ Block: has_zero_or_more
    User ||--o{ Bookmark: has_zero_or_more
    Post ||--o{ PostVote: has_zero_or_more
    Post ||--o{ Report: has_zero_or_more
    Comment ||--o{ CommentVote: has_zero_or_more
    Comment ||--|| CommentNotification: has_one
    Post ||--|| PostNotification: has_one
    User ||--o{ CommentNotification: has-zero_or_more
    User ||--o{ PostNotification: has_zero_or_more
    User ||--o| PasswordResets: has_zero_or_one

PublicPostView {
int post_id
string guild_tag
string link_url
string title
string body
bool is_locked
bool is_edited
time created_at
user_string username
user_string avatar_url
user_bool is_admin
user_bool is_verified
postvote_int up_vote_count
postvote_int down_vote_count
comment_int comment_count
}
PublicCommentView {
int comment_id
int post_id
int parent_comment_id
string body
time timestamp
bool is_edited
user_string username
user_string avatar_url
user_bool is_admin
user_bool is_verified
commentvote_int up_vote_count
commentvote_int down_vote_count
}
PublicGuildView {
string guild_tag
string name
string description
string avatar_url
string banner_url
bool is_banned
time created_at
guildmembership_int member_count
guildmembership_user_string admin_username
guildmembership_user_json mod_usernames
}
