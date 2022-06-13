use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    pub comment_id: i32,
    pub post_id: i32,
    pub user_id: i32,
    pub parent_comment_id: Option<i32>,
    pub body: String,
    pub created_at: String, //convert time to string
    pub is_edited: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommentForm {
    pub post_id: i32,
    pub parent_comment_id: Option<i32>,
    pub user_id: i32,
    pub body: String,
}

impl Comment {
    pub async fn create(
        comment_form: &CommentForm,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        match comment_form.parent_comment_id {
            Some(parent_comment_id) => {
                sqlx::query!(
                    r#"
                    INSERT INTO comments (post_id, parent_comment_id, user_id, body)
                    VALUES ($1, $2, $3, $4)
                    "#,
                    comment_form.post_id,
                    parent_comment_id,
                    comment_form.user_id,
                    comment_form.body
                )
                .execute(tx)
                .await?;
                Ok(())
            }
            None => {
                sqlx::query!(
                    r#"
                    INSERT INTO comments (post_id, user_id, body)
                    VALUES ($1, $2, $3)
                    "#,
                    comment_form.post_id,
                    comment_form.user_id,
                    comment_form.body
                )
                .execute(tx)
                .await?;
                Ok(())
            }
        }
    }

    pub async fn find_by_comment_id(comment_id: &i32, pool: &PgPool) -> Result<Option<Comment>> {
        let comment = sqlx::query!(
            r#"
            SELECT * FROM comments
            WHERE comment_id = $1
            "#,
            comment_id
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(comment.map(|comment| Comment {
            comment_id: comment.comment_id,
            post_id: comment.post_id,
            user_id: comment.user_id,
            parent_comment_id: comment.parent_comment_id,
            body: comment.body,
            created_at: comment.created_at.to_string(), //convert time to string
            is_edited: comment.is_edited,
        }))
    }
    pub async fn find_latest_comments_by_user_id(
        user_id: &i32,
        pool: &PgPool,
        results_per_page: &i64,
        page_number: &i64,
    ) -> Result<Vec<Comment>> {
        let comments = sqlx::query!(
            r#"
            SELECT * FROM comments
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
        .map(|comment| Comment {
            comment_id: comment.comment_id,
            post_id: comment.post_id,
            user_id: comment.user_id,
            parent_comment_id: comment.parent_comment_id,
            body: comment.body,
            created_at: comment.created_at.to_string(), //convert time to string
            is_edited: comment.is_edited,
        })
        .collect();

        Ok(comments)
    }
    pub async fn find_latest_comments_by_post_id(
        post_id: &i32,
        pool: &PgPool,
        results_per_page: &i64,
        page_number: &i64,
    ) -> Result<Vec<Comment>> {
        let comments = sqlx::query!(
            r#"
            SELECT * FROM comments
            WHERE post_id = $1
            ORDER BY created_at
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
        .map(|comment| Comment {
            comment_id: comment.comment_id,
            post_id: comment.post_id,
            user_id: comment.user_id,
            parent_comment_id: comment.parent_comment_id,
            body: comment.body,
            created_at: comment.created_at.to_string(), //convert time to string
            is_edited: comment.is_edited,
        })
        .collect();

        Ok(comments)
    }
    pub async fn update(
        comment_id: &i32,
        new_body: &String,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE comments
            SET body = $2, is_edited = $3
            WHERE comment_id = $1
            "#,
            comment_id,
            new_body,
            true
        )
        .execute(tx)
        .await?;
        Ok(())
    }
    pub async fn delete(comment_id: &i32, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM comments
            WHERE comment_id = $1
            "#,
            comment_id
        )
        .execute(tx)
        .await?;
        Ok(())
    }
}
