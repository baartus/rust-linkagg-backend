use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailedPostView {
    pub post_id: Option<i32>,
    pub guild_tag: Option<String>,
    pub image_url: Option<String>,
    pub link_url: Option<String>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub is_locked: Option<bool>,
    pub is_edited: Option<bool>,
    pub created_at: Option<String>, //time to string
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub is_admin: Option<bool>,
    pub is_verified: Option<bool>,
    pub upvotes: Option<i32>,
    pub downvotes: Option<i32>,
    pub replies: Option<i32>,
    pub is_blocked: bool,
    pub is_upvoted: bool,
    pub is_downvoted: bool,
}
impl DetailedPostView {
    pub async fn get_post_by_id(
        post_id: &i32,
        pool: &PgPool,
        results_per_page: &i64,
        page_number: &i64,
    ) -> Result<Option<DetailedPostView>> {
        let post = sqlx::query!(
            r#"
            SELECT * FROM detailed_post_view
            WHERE post_id = $1
            "#,
            post_id
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(post.map(|post| DetailedPostView {
            post_id: post.post_id,
            guild_tag: post.guild_tag,
            image_url: post.image_url,
            link_url: post.link_url,
            title: post.title,
            body: post.body,
            is_locked: post.is_locked,
            is_edited: post.is_edited,
            created_at: post.created_at.map(|c| c.to_string()),
            username: post.username,
            avatar_url: post.avatar_url,
            is_admin: post.is_admin,
            is_verified: post.is_verified,
            upvotes: post.upvotes,
            downvotes: post.downvotes,
            replies: post.replies,
            is_blocked: false,
            is_upvoted: false,
            is_downvoted: false,
        }))
    }
    pub async fn get_posts_by_guild(
        guild_tag: &String,
        pool: &PgPool,
        results_per_page: &i64,
        page_number: &i64,
    ) -> Result<Vec<DetailedPostView>> {
        let posts = sqlx::query!(
            r#"
            SELECT * FROM detailed_post_view
            WHERE guild_tag = $1
            LIMIT $2
            OFFSET $3
            "#,
            guild_tag,
            results_per_page,
            ((page_number - 1) * results_per_page)
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|post| DetailedPostView {
            post_id: post.post_id,
            guild_tag: post.guild_tag,
            image_url: post.image_url,
            link_url: post.link_url,
            title: post.title,
            body: post.body,
            is_locked: post.is_locked,
            is_edited: post.is_edited,
            created_at: post.created_at.map(|c| c.to_string()),
            username: post.username,
            avatar_url: post.avatar_url,
            is_admin: post.is_admin,
            is_verified: post.is_verified,
            upvotes: post.upvotes,
            downvotes: post.downvotes,
            replies: post.replies,
            is_blocked: false,
            is_upvoted: false,
            is_downvoted: false,
        })
        .collect();

        Ok(posts)
    }
    pub async fn get_all_posts(
        pool: &PgPool,
        results_per_page: &i64,
        page_number: &i64,
    ) -> Result<Vec<DetailedPostView>> {
        let posts = sqlx::query!(
            r#"
            SELECT * FROM detailed_post_view
            LIMIT $1
            OFFSET $2
            "#,
            results_per_page,
            ((page_number - 1) * results_per_page)
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|post| DetailedPostView {
            post_id: post.post_id,
            guild_tag: post.guild_tag,
            image_url: post.image_url,
            link_url: post.link_url,
            title: post.title,
            body: post.body,
            is_locked: post.is_locked,
            is_edited: post.is_edited,
            created_at: post.created_at.map(|c| c.to_string()),
            username: post.username,
            avatar_url: post.avatar_url,
            is_admin: post.is_admin,
            is_verified: post.is_verified,
            upvotes: post.upvotes,
            downvotes: post.downvotes,
            replies: post.replies,
            is_blocked: false,
            is_upvoted: false,
            is_downvoted: false,
        })
        .collect();

        Ok(posts)
    }
    pub async fn get_posts_by_user(
        username: &String,
        pool: &PgPool,
        results_per_page: &i64,
        page_number: &i64,
    ) -> Result<Vec<DetailedPostView>> {
        let posts = sqlx::query!(
            r#"
            SELECT * FROM detailed_post_view
            WHERE username = $1
            LIMIT $2
            OFFSET $3
            "#,
            username,
            results_per_page,
            ((page_number - 1) * results_per_page)
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|post| DetailedPostView {
            post_id: post.post_id,
            guild_tag: post.guild_tag,
            image_url: post.image_url,
            link_url: post.link_url,
            title: post.title,
            body: post.body,
            is_locked: post.is_locked,
            is_edited: post.is_edited,
            created_at: post.created_at.map(|c| c.to_string()),
            username: post.username,
            avatar_url: post.avatar_url,
            is_admin: post.is_admin,
            is_verified: post.is_verified,
            upvotes: post.upvotes,
            downvotes: post.downvotes,
            replies: post.replies,
            is_blocked: false,
            is_upvoted: false,
            is_downvoted: false,
        })
        .collect();

        Ok(posts)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailedCommentView {
    pub comment_id: Option<i32>,
    pub post_id: Option<i32>,
    pub parent_comment_id: Option<i32>,
    pub body: Option<String>,
    pub is_edited: Option<bool>,
    pub created_at: Option<String>, //time to string
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub is_admin: Option<bool>,
    pub is_verified: Option<bool>,
    pub upvotes: Option<i32>,
    pub downvotes: Option<i32>,
    pub is_blocked: bool,
    pub is_upvoted: bool,
    pub is_downvoted: bool,
}
impl DetailedCommentView {
    pub async fn get_comment_by_id(
        comment_id: &i32,
        pool: &PgPool,
    ) -> Result<Option<DetailedCommentView>> {
        let comment = sqlx::query!(
            r#"
            SELECT * FROM detailed_comment_view
            WHERE comment_id = $1
            "#,
            comment_id
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(comment.map(|comment| DetailedCommentView {
            comment_id: comment.comment_id,
            post_id: comment.post_id,
            parent_comment_id: comment.parent_comment_id,
            body: comment.body,
            is_edited: comment.is_edited,
            created_at: comment.created_at.map(|c| c.to_string()),
            username: comment.username,
            avatar_url: comment.avatar_url,
            is_admin: comment.is_admin,
            is_verified: comment.is_verified,
            upvotes: comment.upvotes,
            downvotes: comment.downvotes,
            is_blocked: false,
            is_upvoted: false,
            is_downvoted: false,
        }))
    }
    pub async fn get_comments_by_post_id(
        post_id: &i32,
        pool: &PgPool,
        results_per_page: &i64,
        page_number: &i64,
    ) -> Result<Vec<DetailedCommentView>> {
        let comments = sqlx::query!(
            r#"
            SELECT * FROM detailed_comment_view
            WHERE post_id = $1
            LIMIT $2
            OFFSET $3
            "#,
            post_id,
            results_per_page,
            ((page_number - 1) * results_per_page)
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|comment| DetailedCommentView {
            comment_id: comment.comment_id,
            post_id: comment.post_id,
            parent_comment_id: comment.parent_comment_id,
            body: comment.body,
            is_edited: comment.is_edited,
            created_at: comment.created_at.map(|c| c.to_string()),
            username: comment.username,
            avatar_url: comment.avatar_url,
            is_admin: comment.is_admin,
            is_verified: comment.is_verified,
            upvotes: comment.upvotes,
            downvotes: comment.downvotes,
            is_blocked: false,
            is_upvoted: false,
            is_downvoted: false,
        })
        .collect();

        Ok(comments)
    }

    pub async fn get_comments_by_username(
        username: &String,
        pool: &PgPool,
        results_per_page: &i64,
        page_number: &i64,
    ) -> Result<Vec<DetailedCommentView>> {
        let comments = sqlx::query!(
            r#"
            SELECT * FROM detailed_comment_view
            WHERE username = $1
            LIMIT $2
            OFFSET $3
            "#,
            username,
            results_per_page,
            ((page_number - 1) * results_per_page)
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|comment| DetailedCommentView {
            comment_id: comment.comment_id,
            post_id: comment.post_id,
            parent_comment_id: comment.parent_comment_id,
            body: comment.body,
            is_edited: comment.is_edited,
            created_at: comment.created_at.map(|c| c.to_string()),
            username: comment.username,
            avatar_url: comment.avatar_url,
            is_admin: comment.is_admin,
            is_verified: comment.is_verified,
            upvotes: comment.upvotes,
            downvotes: comment.downvotes,
            is_blocked: false,
            is_upvoted: false,
            is_downvoted: false,
        })
        .collect();

        Ok(comments)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailedGuildView {
    pub guild_tag: Option<String>,
    pub guild_name: Option<String>,
    pub guild_description: Option<String>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub is_banned: Option<bool>,
    pub created_at: Option<String>, //convert time to string
    pub members: Option<i32>,
    pub number_of_posts: Option<i32>,
}
impl DetailedGuildView {
    pub async fn find_by_guild_tag(
        guild_tag: &String,
        pool: &PgPool,
    ) -> Result<Option<DetailedGuildView>> {
        let guild = sqlx::query!(
            r#"
            SELECT * FROM detailed_guild_view
            WHERE guild_tag = $1
            "#,
            guild_tag
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(guild.map(|guild| DetailedGuildView {
            guild_tag: guild.guild_tag,
            guild_name: guild.guild_name,
            guild_description: guild.guild_description,
            avatar_url: guild.avatar_url,
            banner_url: guild.banner_url,
            is_banned: guild.is_banned,
            created_at: guild.created_at.map(|c| c.to_string()),
            members: guild.members,
            number_of_posts: guild.number_of_posts,
        }))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShortGuildView {
    pub guild_tag: Option<String>,
    pub guild_name: Option<String>,
    pub avatar_url: Option<String>,
    pub members: Option<i32>,
    pub number_of_posts: Option<i32>,
    pub is_member: bool,
}
impl ShortGuildView {
    pub async fn find_all(
        results_per_page: &i64,
        page_number: &i64,
        pool: &PgPool,
    ) -> Result<Vec<ShortGuildView>> {
        let guilds = sqlx::query!(
            r#"
            SELECT * FROM short_guild_view
            LIMIT $1
            OFFSET $2
            "#,
            results_per_page,
            ((page_number - 1) * results_per_page)
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|guild| ShortGuildView {
            guild_tag: guild.guild_tag,
            guild_name: guild.guild_name,
            avatar_url: guild.avatar_url,
            members: guild.members,
            number_of_posts: guild.number_of_posts,
            is_member: false,
        })
        .collect();

        Ok(guilds)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailedUserView {
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub is_admin: Option<bool>,
    pub is_verified: Option<bool>,
    pub is_banned: Option<bool>,
    pub created_at: Option<String>, //time to string
    pub upvotes: Option<i32>,
    pub downvotes: Option<i32>,
    pub number_of_posts: Option<i32>,
    pub number_of_comments: Option<i32>,
    pub number_of_memberships: Option<i32>,
}
impl DetailedUserView {
    pub async fn find_by_username(
        username: &String,
        pool: &PgPool,
    ) -> Result<Option<DetailedUserView>> {
        let user = sqlx::query!(
            r#"
            SELECT * FROM detailed_user_view
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(user.map(|user| DetailedUserView {
            username: user.username,
            avatar_url: user.avatar_url,
            is_admin: user.is_admin,
            is_verified: user.is_verified,
            is_banned: user.is_banned,
            created_at: user.created_at.map(|c| c.to_string()),
            upvotes: user.upvotes,
            downvotes: user.downvotes,
            number_of_posts: user.number_of_posts,
            number_of_comments: user.number_of_comments,
            number_of_memberships: user.number_of_memberships,
        }))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPersonalView {
    pub username: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}
impl UserPersonalView {
    pub async fn find_by_username(
        username: &String,
        pool: &PgPool,
    ) -> Result<Option<UserPersonalView>> {
        let user = sqlx::query!(
            r#"
            SELECT * FROM user_personal_view
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(user.map(|user| UserPersonalView {
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
        }))
    }
}
