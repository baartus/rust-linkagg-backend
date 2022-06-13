use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub post_id: i32,
    pub guild_tag: String,
    pub user_id: i32,
    pub image_url: Option<String>,
    pub link_url: Option<String>,
    pub title: String,
    pub body: Option<String>, //Markdown text
    pub is_locked: bool,
    pub is_edited: bool,
    pub created_at: String, //convert time to string
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostForm {
    pub guild_tag: String,
    pub user_id: i32,
    pub image_url: Option<String>,
    pub link_url: Option<String>,
    pub title: String,
    pub body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostEditForm {
    pub post_id: i32,
    pub new_image_url: Option<String>,
    pub new_link_url: Option<String>,
    pub new_title: String,
    pub new_body: Option<String>,
}

impl Post {
    pub async fn create(post_form: &PostForm, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO posts (guild_tag, user_id, image_url, link_url, title, body)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            post_form.guild_tag,
            post_form.user_id,
            post_form.image_url,
            post_form.link_url,
            post_form.title,
            post_form.body
        )
        .execute(tx)
        .await?;
        Ok(())
    }
    pub async fn find_by_post_id(post_id: &i32, pool: &PgPool) -> Result<Option<Post>> {
        let post = sqlx::query!(
            r#"
            SELECT * FROM posts
            WHERE post_id = $1
            "#,
            post_id
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(post.map(|post| Post {
            post_id: post.post_id,
            guild_tag: post.guild_tag,
            user_id: post.user_id,
            image_url: post.image_url,
            link_url: post.link_url,
            title: post.title,
            body: post.body, //Markdown text
            is_locked: post.is_locked,
            is_edited: post.is_edited,
            created_at: post.created_at.to_string(), //convert time to string
        }))
    }
    pub async fn find_latest_posts_by_user_id(
        user_id: &i32,
        pool: &PgPool,
        results_per_page: &i64,
        page_number: &i64,
    ) -> Result<Vec<Post>> {
        let posts = sqlx::query!(
            r#"
            SELECT * FROM posts
            WHERE user_id = $1
            ORDER BY created_at
            LIMIT $2
            OFFSET $3
            "#,
            user_id,
            results_per_page,
            ((page_number - 1) * results_per_page)
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|post| Post {
            post_id: post.post_id,
            guild_tag: post.guild_tag,
            user_id: post.user_id,
            image_url: post.image_url,
            link_url: post.link_url,
            title: post.title,
            body: post.body, //Markdown text
            is_locked: post.is_locked,
            is_edited: post.is_edited,
            created_at: post.created_at.to_string(), //convert time to string
        })
        .collect();

        Ok(posts)
    }
    pub async fn find_latest_posts_by_guild_id(
        guild_tag: &String,
        pool: &PgPool,
        results_per_page: &i64,
        page_number: &i64,
    ) -> Result<Vec<Post>> {
        let posts = sqlx::query!(
            r#"
            SELECT * FROM posts
            WHERE guild_tag = $1
            ORDER BY created_at
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
        .map(|post| Post {
            post_id: post.post_id,
            guild_tag: post.guild_tag,
            user_id: post.user_id,
            image_url: post.image_url,
            link_url: post.link_url,
            title: post.title,
            body: post.body, //Markdown text
            is_locked: post.is_locked,
            is_edited: post.is_edited,
            created_at: post.created_at.to_string(), //convert time to string
        })
        .collect();

        Ok(posts)
    }
    pub async fn update(edits: &PostEditForm, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE posts
            SET link_url = $2, title = $3, body = $4, is_edited = $5, image_url = $6
            WHERE post_id = $1
            "#,
            edits.post_id,
            edits.new_link_url,
            edits.new_title,
            edits.new_body,
            true,
            edits.new_image_url
        )
        .execute(tx)
        .await?;
        Ok(())
    }
    pub async fn update_lock(
        post_id: &i32,
        locked: bool,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE posts
            SET is_locked = $2
            WHERE post_id = $1
            "#,
            post_id,
            locked
        )
        .execute(tx)
        .await?;
        Ok(())
    }
    pub async fn delete(post_id: &i32, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM posts
            WHERE post_id = $1
            "#,
            post_id
        )
        .execute(tx)
        .await?;
        Ok(())
    }
}
