use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSession {
    pub session_id: String,
    pub user_id: i32,
    pub created_at: String, //convert time to string
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSessionView {
    pub session_id: String,
    pub user_id: i32,
}

impl UserSession {
    pub async fn create(
        user_id: i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<UserSessionView> {
        let session_id = Uuid::new_v4().to_string();
        //insert session into table
        sqlx::query!(
            r#"
                INSERT INTO user_sessions (session_id, user_id)
                VALUES ($1, $2)
            "#,
            &session_id,
            &user_id,
        )
        .execute(tx)
        .await?;

        let new_session = UserSessionView {
            session_id,
            user_id,
        };

        Ok(new_session)
    }
    pub async fn find_by_session_id(
        session_id: String,
        pool: &PgPool,
    ) -> Result<Option<UserSession>> {
        let session = sqlx::query!(
            r#"
            SELECT *
            FROM user_sessions
            WHERE session_id = $1
            "#,
            session_id
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(session.map(|session| UserSession {
            session_id: session.session_id,
            user_id: session.user_id,
            created_at: session.created_at.to_string(),
        }))
    }
    pub async fn find_sessions_by_user_id(user_id: i32, pool: &PgPool) -> Result<Vec<UserSession>> {
        let sessions = sqlx::query!(
            r#"
            SELECT *
            FROM user_sessions
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|session| UserSession {
            session_id: session.session_id,
            user_id: session.user_id,
            created_at: session.created_at.to_string(),
        })
        .collect();

        Ok(sessions)
    }
    pub async fn delete(session: UserSession, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM user_sessions
            WHERE session_id = $1
            "#,
            session.session_id,
        )
        .execute(tx)
        .await?;
        Ok(())
    }
}
