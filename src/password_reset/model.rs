use anyhow::Result;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PasswordReset {
    pub reset_hash: String,
    pub user_id: i32,
    pub verified_email: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PasswordResetForm {
    pub new_password: String,
    pub confirm_new_password: String,
}

impl PasswordReset {
    pub async fn create_reset(user_id: &i32, tx: &mut Transaction<'_, Postgres>) -> Result<String> {
        //generate random string for reset url
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        abcdefghijklmnopqrstuvwxyz";
        let mut rng = rand::thread_rng();
        let reset_hash: String = (0..25)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        sqlx::query!(
            r#"
            INSERT INTO password_resets (reset_hash, user_id)
            VALUES ($1, $2)
            "#,
            &reset_hash,
            user_id
        )
        .execute(tx)
        .await?;
        Ok(reset_hash)
    }
    pub async fn delete(user_id: &i32, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM password_resets
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(tx)
        .await?;
        Ok(())
    }
    pub async fn find_reset_by_hash(
        reset_hash: &String,
        pool: &PgPool,
    ) -> Result<Option<PasswordReset>> {
        let reset = sqlx::query!(
            r#"
            SELECT * FROM password_resets
            WHERE reset_hash = $1
            "#,
            reset_hash
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(reset.map(|reset| PasswordReset {
            reset_hash: reset.reset_hash,
            user_id: reset.user_id,
            verified_email: reset.verified_email,
        }))
    }
    pub async fn verify_reset(user_id: &i32, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        let verified_reset = sqlx::query!(
            r#"
            UPDATE password_resets
            SET verified_email = true
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(tx)
        .await?;
        Ok(())
    }
}
