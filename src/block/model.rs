use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub user_id: i32,
    pub blocked_user_username: String,
}

impl Block {
    pub async fn create(
        user_id: &i32,
        blocked_user_username: &String,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO blocks (user_id, blocked_user_username)
            VALUES ($1, $2)
            "#,
            user_id,
            blocked_user_username
        )
        .execute(tx)
        .await?;
        Ok(())
    }
    pub async fn find_all_by_user(user_id: &i32, pool: &PgPool) -> Result<Vec<Block>> {
        let blocks = sqlx::query!(
            r#"
            SELECT * FROM blocks
            WHERE user_id = $1
            "#,
            user_id,
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|block| Block {
            user_id: block.user_id,
            blocked_user_username: block.blocked_user_username,
        })
        .collect();
        Ok(blocks)
    }
    pub async fn find_by_user_and_blocked_user_username(
        user_id: &i32,
        blocked_user_username: &String,
        pool: &PgPool,
    ) -> Result<Option<Block>> {
        let block = sqlx::query!(
            r#"
            SELECT * FROM blocks
            WHERE user_id = $1 AND blocked_user_username = $2
            "#,
            user_id,
            blocked_user_username
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(block.map(|block| Block {
            user_id: block.user_id,
            blocked_user_username: block.blocked_user_username,
        }))
    }
    pub async fn delete(
        user_id: &i32,
        blocked_user_username: &String,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM blocks
            WHERE user_id = $1 AND blocked_user_username = $2
            "#,
            user_id,
            blocked_user_username
        )
        .execute(tx)
        .await?;
        Ok(())
    }
}
