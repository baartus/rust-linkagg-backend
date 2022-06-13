use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CommentNotificationType {
    ReplyNotification,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommentNotification {
    pub notification_id: i32,
    pub notification_type: String,
    pub user_id: i32,
    pub comment_id: i32,
    pub is_read: bool,      //the comment or post id that the notification is about
    pub created_at: String, //convert time to string
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommentNotificationForm {
    pub notification_type: String,
    pub user_id: i32,
    pub target_id: i32,
}

impl CommentNotification {
    pub async fn find_by_id(
        notification_id: &i32,
        pool: &PgPool,
    ) -> Result<Option<CommentNotification>> {
        let noti = sqlx::query!(
            r#"
            SELECT * FROM comment_notifications
            WHERE notification_id = $1
            "#,
            notification_id
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(noti.map(|noti| CommentNotification {
            notification_id: noti.notification_id,
            notification_type: noti.notification_type.to_string(),
            user_id: noti.user_id,
            comment_id: noti.comment_id,
            is_read: noti.is_read,
            created_at: noti.created_at.to_string(),
        }))
    }
    pub async fn find_notis_by_user_id(
        user_id: &i32,
        pool: &PgPool,
        results_per_page: &i64,
        page_number: &i64,
    ) -> Result<Vec<CommentNotification>> {
        let notis = sqlx::query!(
            r#"
            SELECT * FROM comment_notifications
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
        .map(|noti| CommentNotification {
            notification_id: noti.notification_id,
            notification_type: noti.notification_type,
            user_id: noti.user_id,
            comment_id: noti.comment_id,
            is_read: noti.is_read,
            created_at: noti.created_at.to_string(),
        })
        .collect();
        Ok(notis)
    }
    pub async fn delete(notification_id: &i32, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM comment_notifications
            WHERE notification_id = $1
            "#,
            notification_id
        )
        .execute(tx)
        .await?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PostNotificationType {
    GuildPostNotification,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostNotification {
    pub notification_id: i32,
    pub notification_type: String,
    pub user_id: i32,
    pub post_id: i32,
    pub is_read: bool,      //the comment or post id that the notification is about
    pub created_at: String, //convert time to string
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostNotificationForm {
    pub notification_type: PostNotificationType,
    pub user_id: i32,
    pub target_id: i32,
}

impl PostNotification {
    pub async fn find_by_id(
        notification_id: &i32,
        pool: &PgPool,
    ) -> Result<Option<PostNotification>> {
        let noti = sqlx::query!(
            r#"
            SELECT * FROM post_notifications
            WHERE notification_id = $1
            "#,
            notification_id
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(noti.map(|noti| PostNotification {
            notification_id: noti.notification_id,
            notification_type: noti.notification_type,
            user_id: noti.user_id,
            post_id: noti.post_id,
            is_read: noti.is_read,
            created_at: noti.created_at.to_string(),
        }))
    }
    pub async fn find_notis_by_user_id(
        user_id: &i32,
        pool: &PgPool,
        results_per_page: &i64,
        page_number: &i64,
    ) -> Result<Vec<PostNotification>> {
        let notis = sqlx::query!(
            r#"
            SELECT * FROM post_notifications
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
        .map(|noti| PostNotification {
            notification_id: noti.notification_id,
            notification_type: noti.notification_type,
            user_id: noti.user_id,
            post_id: noti.post_id,
            is_read: noti.is_read,
            created_at: noti.created_at.to_string(),
        })
        .collect();
        Ok(notis)
    }
    pub async fn delete(notification_id: &i32, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM post_notifications
            WHERE notification_id = $1
            "#,
            notification_id
        )
        .execute(tx)
        .await?;
        Ok(())
    }
}
