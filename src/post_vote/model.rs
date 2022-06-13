use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostVote {
    pub post_id: i32,
    pub user_id: i32,
    pub up: bool,
}

impl PostVote {
    pub async fn create(post_vote: &PostVote, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO post_votes (post_id, user_id, up)
            VALUES ($1, $2, $3)
            "#,
            post_vote.post_id,
            post_vote.user_id,
            post_vote.up
        )
        .execute(tx)
        .await?;
        Ok(())
    }
    pub async fn update(updated_vote: &PostVote, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE post_votes 
            SET up = $3
            WHERE post_id = $1 AND user_id = $2
            "#,
            updated_vote.post_id,
            updated_vote.user_id,
            updated_vote.up,
        )
        .execute(tx)
        .await?;
        Ok(())
    }
    pub async fn find_all_by_post_id(post_id: &i32, pool: &PgPool) -> Result<Vec<PostVote>> {
        let votes = sqlx::query!(
            r#"
            SELECT * FROM post_votes
            WHERE post_id = $1
            "#,
            post_id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|vote| PostVote {
            post_id: vote.post_id,
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
    ) -> Result<Vec<PostVote>> {
        let votes = sqlx::query!(
            r#"
            SELECT * FROM post_votes
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
        .map(|vote| PostVote {
            post_id: vote.post_id,
            user_id: vote.user_id,
            up: vote.up,
        })
        .collect();

        Ok(votes)
    }
    pub async fn find_by_post_and_user_id(
        post_id: &i32,
        user_id: &i32,
        pool: &PgPool,
    ) -> Result<Option<PostVote>> {
        let vote = sqlx::query!(
            r#"
            SELECT * FROM post_votes
            WHERE post_id = $1 AND user_id = $2
            "#,
            post_id,
            user_id
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(vote.map(|vote| PostVote {
            post_id: vote.post_id,
            user_id: vote.user_id,
            up: vote.up,
        }))
    }
    pub async fn delete(
        post_id: &i32,
        user_id: &i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM post_votes
            WHERE post_id = $1 AND user_id = $2
            "#,
            post_id,
            user_id
        )
        .execute(tx)
        .await?;
        Ok(())
    }
}
