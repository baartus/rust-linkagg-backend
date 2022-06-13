use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommentVote {
    pub comment_id: i32,
    pub user_id: i32,
    pub up: bool,
}

impl CommentVote {
    pub async fn create(
        comment_vote: &CommentVote,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO comment_votes (comment_id, user_id, up)
            VALUES ($1, $2, $3)
            "#,
            comment_vote.comment_id,
            comment_vote.user_id,
            comment_vote.up
        )
        .execute(tx)
        .await?;
        Ok(())
    }
    pub async fn update(
        updated_vote: &CommentVote,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE comment_votes 
            SET up = $3
            WHERE comment_id = $1 AND user_id = $2
            "#,
            updated_vote.comment_id,
            updated_vote.user_id,
            updated_vote.up,
        )
        .execute(tx)
        .await?;
        Ok(())
    }
    pub async fn find_all_by_comment_id(
        comment_id: &i32,
        pool: &PgPool,
    ) -> Result<Vec<CommentVote>> {
        let votes = sqlx::query!(
            r#"
            SELECT * FROM comment_votes
            WHERE comment_id = $1
            "#,
            comment_id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|vote| CommentVote {
            comment_id: vote.comment_id,
            user_id: vote.user_id,
            up: vote.up,
        })
        .collect();

        Ok(votes)
    }
    pub async fn find_all_by_user_id(
        user_id: &i32,
        pool: &PgPool,
        results_per_page: &i64,
        page_number: &i64,
    ) -> Result<Vec<CommentVote>> {
        let votes = sqlx::query!(
            r#"
            SELECT * FROM comment_votes
            WHERE user_id = $1
            LIMIT $2
            OFFSET $3
            "#,
            user_id,
            results_per_page,
            page_number,
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|vote| CommentVote {
            comment_id: vote.comment_id,
            user_id: vote.user_id,
            up: vote.up,
        })
        .collect();

        Ok(votes)
    }
    pub async fn find_by_comment_and_user_id(
        comment_id: &i32,
        user_id: &i32,
        pool: &PgPool,
    ) -> Result<Option<CommentVote>> {
        let vote = sqlx::query!(
            r#"
            SELECT * FROM comment_votes
            WHERE comment_id = $1 AND user_id = $2
            "#,
            comment_id,
            user_id
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(vote.map(|vote| CommentVote {
            comment_id: vote.comment_id,
            user_id: vote.user_id,
            up: vote.up,
        }))
    }
    pub async fn delete(
        comment_id: &i32,
        user_id: &i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM comment_votes
            WHERE comment_id = $1 AND user_id = $2
            "#,
            comment_id,
            user_id
        )
        .execute(tx)
        .await?;
        Ok(())
    }
}
