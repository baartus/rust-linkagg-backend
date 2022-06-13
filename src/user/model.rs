use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::Postgres;
use sqlx::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub user_id: i32,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub avatar_url: Option<String>,
    pub is_admin: bool,
    pub is_verified: bool,
    pub is_banned: bool,
    pub created_at: String, //convert time to string
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserView {
    pub username: String,
    pub avatar_url: Option<String>,
    pub is_admin: bool,
    pub is_verified: bool,
    pub is_banned: bool,
    pub created_at: String, //convert time to string
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserViewPersonal {
    pub email: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub is_admin: bool,
    pub is_verified: bool,
    pub is_banned: bool,
    pub created_at: String, //convert time to string
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserCreateForm {
    pub email: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserLoginForm {
    pub username: String,
    pub password: String,
}

impl User {
    pub async fn verify_password(user: &User, password_input: &String) -> Result<bool> {
        let valid = verify(password_input, &user.password_hash).unwrap_or(false);
        Ok(valid)
    }
    pub async fn create(
        user_form: &UserCreateForm,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO users (email, username, password_hash)
                VALUES ($1, $2, $3)
            "#,
            user_form.email,
            user_form.username,
            user_form.password_hash,
        )
        .execute(tx)
        .await?;
        Ok(())
    }

    pub async fn find_by_username_sensitive(
        username: &String,
        pool: &PgPool,
    ) -> Result<Option<User>> {
        let user = sqlx::query!(
            r#"
                SELECT * FROM users
                WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(user.map(|user| User {
            user_id: user.user_id,
            email: user.email,
            username: user.username,
            password_hash: user.password_hash,
            avatar_url: user.avatar_url,
            is_admin: user.is_admin,
            is_verified: user.is_verified,
            is_banned: user.is_banned,
            created_at: user.created_at.to_string(),
        }))
    }

    pub async fn find_by_username(username: &String, pool: &PgPool) -> Result<Option<UserView>> {
        let user = sqlx::query!(
            r#"
                SELECT * FROM users
                WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(user.map(|user| UserView {
            username: user.username,
            avatar_url: user.avatar_url,
            is_admin: user.is_admin,
            is_verified: user.is_verified,
            is_banned: user.is_banned,
            created_at: user.created_at.to_string(),
        }))
    }
    pub async fn find_by_id(id: &i32, pool: &PgPool) -> Result<Option<User>> {
        let user = sqlx::query!(
            r#"
                SELECT * FROM users
                WHERE user_id = $1
            "#,
            id
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(user.map(|user| User {
            user_id: user.user_id,
            email: user.email,
            username: user.username,
            password_hash: user.password_hash,
            avatar_url: user.avatar_url,
            is_admin: user.is_admin,
            is_verified: user.is_verified,
            is_banned: user.is_banned,
            created_at: user.created_at.to_string(),
        }))
    }
    pub fn to_safe_view(user: User) -> UserView {
        return UserView {
            username: user.username,
            avatar_url: user.avatar_url,
            is_admin: user.is_admin,
            is_verified: user.is_verified,
            is_banned: user.is_banned,
            created_at: user.created_at,
        };
    }
    pub async fn find_by_email(email: &String, pool: &PgPool) -> Result<Option<UserView>> {
        let user = sqlx::query!(
            r#"
                SELECT * FROM users
                WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(user.map(|user| UserView {
            username: user.username,
            avatar_url: user.avatar_url,
            is_admin: user.is_admin,
            is_verified: user.is_verified,
            is_banned: user.is_banned,
            created_at: user.created_at.to_string(),
        }))
    }
    pub async fn update_username(
        new_username: String,
        user_id: &i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET username = $2
            WHERE user_id = $1
            "#,
            user_id,
            new_username
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn update_email(
        new_email: String,
        user_id: &i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET email = $2
            WHERE user_id = $1
            "#,
            user_id,
            new_email
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn update_password(
        new_password: &String,
        user_id: &i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        let new_password_hash = hash(new_password, DEFAULT_COST).expect("Couldn't hash password");
        sqlx::query!(
            r#"
            UPDATE users
            SET password_hash = $2
            WHERE user_id = $1
            "#,
            user_id,
            new_password_hash
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn update_avatar_url(
        new_avatar_url: String,
        user_id: &i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET avatar_url = $2
            WHERE user_id = $1
            "#,
            user_id,
            new_avatar_url
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn update_admin_status(
        new_admin_status: bool,
        user_id: &i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET is_admin = $2
            WHERE user_id = $1
            "#,
            user_id,
            new_admin_status
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn update_verified_status(
        new_verified_status: bool,
        user_id: &i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET is_verified = $2
            WHERE user_id = $1
            "#,
            user_id,
            new_verified_status
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn update_banned_status(
        new_banned_status: bool,
        user_id: &i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET is_banned = $2
            WHERE user_id = $1
            "#,
            user_id,
            new_banned_status
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn delete(user_id: &i32, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE user_id = $1
            "#,
            user_id,
        )
        .execute(tx)
        .await?;
        Ok(())
    }
}
