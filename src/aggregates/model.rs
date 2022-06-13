use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAggregates {
    pub user_id: i32,
    pub upvotes: i32,
    pub downvotes: i32,
    pub number_of_posts: i32,
    pub number_of_comments: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostAggregates {
    pub post_id: i32,
    pub upvotes: i32,
    pub downvotes: i32,
    pub replies: i32,
}

pub struct CommentAggregates {
    pub comment_id: i32,
    pub upvotes: i32,
    pub downvotes: i32,
}

pub struct GuildAggregates {
    pub guild_tag: String,
    pub members: i32,
    pub number_of_posts: i32,
}
