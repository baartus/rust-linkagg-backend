use anyhow::Result;
use bcrypt::{hash, DEFAULT_COST};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::Postgres;
use sqlx::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRegistration {
    pub registration_id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub registration_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRegistrationView {
    pub email: String,
    pub registration_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRegistrationForm {
    pub email: String,
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

impl UserRegistration {
    pub async fn create(
        registration_form: &UserRegistrationForm,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<UserRegistrationView> {
        //hash password
        let password_hash =
            hash(&registration_form.password, DEFAULT_COST).expect("Couldn't hash password");

        //generate random string for registration url
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz";

        let mut rng = rand::thread_rng();

        let registration_hash: String = (0..25)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        sqlx::query!(
            r#"
                INSERT INTO user_registrations (email, username, password_hash, registration_hash)
                VALUES ($1, $2, $3, $4)
            "#,
            &registration_form.email,
            registration_form.username,
            password_hash,
            &registration_hash,
        )
        .execute(tx)
        .await?;

        let response = UserRegistrationView {
            email: registration_form.clone().email,
            registration_hash: registration_hash,
        };

        Ok(response)
    }
    pub async fn find_by_email(email: &String, pool: &PgPool) -> Result<Option<UserRegistration>> {
        let registration = sqlx::query!(
            r#"
            SELECT *
            FROM user_registrations
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(registration.map(|registration| UserRegistration {
            registration_id: registration.registration_id,
            username: registration.username,
            email: registration.email,
            password_hash: registration.password_hash,
            registration_hash: registration.registration_hash,
        }))
    }

    pub async fn find_by_username(
        username: &String,
        pool: &PgPool,
    ) -> Result<Option<UserRegistration>> {
        let registration = sqlx::query!(
            r#"
            SELECT *
            FROM user_registrations
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(registration.map(|registration| UserRegistration {
            registration_id: registration.registration_id,
            username: registration.username,
            email: registration.email,
            password_hash: registration.password_hash,
            registration_hash: registration.registration_hash,
        }))
    }

    pub async fn find_by_hash(
        registration_hash: String,
        pool: &PgPool,
    ) -> Result<Option<UserRegistration>> {
        let registration = sqlx::query!(
            r#"
            SELECT *
            FROM user_registrations
            WHERE registration_hash = $1
            "#,
            registration_hash
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(registration.map(|registration| UserRegistration {
            registration_id: registration.registration_id,
            username: registration.username,
            email: registration.email,
            password_hash: registration.password_hash,
            registration_hash: registration.registration_hash,
        }))
    }
    pub async fn delete(
        registration: UserRegistration,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM user_registrations
            WHERE registration_id = $1
            "#,
            registration.registration_id,
        )
        .execute(tx)
        .await?;
        Ok(())
    }
}
