use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildMembership {
    pub membership_id: i32,
    pub user_id: i32,
    pub guild_tag: String,
    pub is_admin: bool,
    pub is_moderator: bool,
    pub is_banned: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildMembershipForm {
    pub user_id: i32,
    pub guild_tag: String,
}

impl GuildMembership {
    pub async fn create(
        guild_membership_form: &GuildMembershipForm,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO guild_memberships (user_id, guild_tag)
                VALUES ($1, $2)
            "#,
            guild_membership_form.user_id,
            guild_membership_form.guild_tag,
        )
        .execute(tx)
        .await?;
        Ok(())
    }
    pub async fn create_as_admin(
        guild_membership_form: &GuildMembershipForm,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO guild_memberships (user_id, guild_tag, is_admin)
                VALUES ($1, $2, $3)
            "#,
            guild_membership_form.user_id,
            guild_membership_form.guild_tag,
            true
        )
        .execute(tx)
        .await?;
        Ok(())
    }

    pub async fn find_by_membership_id(
        membership_id: &i32,
        pool: &PgPool,
    ) -> Result<Option<GuildMembership>> {
        let membership = sqlx::query!(
            r#"
            SELECT * FROM guild_memberships
            WHERE membership_id = $1
            "#,
            membership_id
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(membership.map(|membership| GuildMembership {
            membership_id: membership.membership_id,
            user_id: membership.user_id,
            guild_tag: membership.guild_tag,
            is_admin: membership.is_admin,
            is_moderator: membership.is_moderator,
            is_banned: membership.is_banned,
        }))
    }

    pub async fn find_all_by_user_id(
        user_id: &i32,
        pool: &PgPool,
        results_per_page: &i64,
        page_number: &i64,
    ) -> Result<Vec<GuildMembership>> {
        if results_per_page == &0 {
            //this if is needed for short guild details list
            let memberships = sqlx::query!(
                r#"
                SELECT *
                FROM guild_memberships
                WHERE user_id = $1
                ORDER BY membership_id
                "#,
                user_id
            )
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|membership| GuildMembership {
                membership_id: membership.membership_id,
                user_id: membership.user_id,
                guild_tag: membership.guild_tag,
                is_admin: membership.is_admin,
                is_moderator: membership.is_moderator,
                is_banned: membership.is_banned,
            })
            .collect();
            Ok(memberships)
        } else {
            let memberships = sqlx::query!(
                r#"
                SELECT *
                FROM guild_memberships
                WHERE user_id = $1
                ORDER BY membership_id
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
            .map(|membership| GuildMembership {
                membership_id: membership.membership_id,
                user_id: membership.user_id,
                guild_tag: membership.guild_tag,
                is_admin: membership.is_admin,
                is_moderator: membership.is_moderator,
                is_banned: membership.is_banned,
            })
            .collect();
            Ok(memberships)
        }
    }

    //paginated
    pub async fn find_all_by_guild_tag(
        guild_tag: &String,
        pool: &PgPool,
        results_per_page: &i64,
        page_number: &i64,
    ) -> Result<Vec<GuildMembership>> {
        let memberships = sqlx::query!(
            r#"
            SELECT *
            FROM guild_memberships
            WHERE guild_tag = $1
            ORDER BY membership_id
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
        .map(|membership| GuildMembership {
            membership_id: membership.membership_id,
            user_id: membership.user_id,
            guild_tag: membership.guild_tag,
            is_admin: membership.is_admin,
            is_moderator: membership.is_moderator,
            is_banned: membership.is_banned,
        })
        .collect();

        Ok(memberships)
    }

    pub async fn find_by_user_and_guild_tag(
        user_id: &i32,
        guild_tag: &String,
        pool: &PgPool,
    ) -> Result<Option<GuildMembership>> {
        let membership = sqlx::query!(
            r#"
            SELECT * FROM guild_memberships
            WHERE user_id = $1 AND guild_tag = $2
            "#,
            user_id,
            guild_tag
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(membership.map(|membership| GuildMembership {
            membership_id: membership.membership_id,
            user_id: membership.user_id,
            guild_tag: membership.guild_tag,
            is_admin: membership.is_admin,
            is_moderator: membership.is_moderator,
            is_banned: membership.is_banned,
        }))
    }

    //update admin status
    pub async fn update_membership_admin_status(
        new_admin_status: bool,
        old_membership: GuildMembership,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE guild_memberships
            SET is_admin = $2
            WHERE membership_id = $1
            "#,
            old_membership.membership_id,
            new_admin_status
        )
        .execute(tx)
        .await?;

        Ok(())
    }
    //update moderator status
    pub async fn update_membership_mod_status(
        new_mod_status: bool,
        old_membership: GuildMembership,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE guild_memberships
            SET is_moderator = $2
            WHERE membership_id = $1
            "#,
            old_membership.membership_id,
            new_mod_status
        )
        .execute(tx)
        .await?;

        Ok(())
    }
    //update banned status
    pub async fn update_membership_ban_status(
        new_ban_status: bool,
        old_membership: GuildMembership,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE guild_memberships
            SET is_banned = $2
            WHERE membership_id = $1
            "#,
            old_membership.membership_id,
            new_ban_status
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn delete(
        guild_membership: GuildMembership,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM guild_memberships
            where membership_id = $1
            "#,
            guild_membership.membership_id
        )
        .execute(tx)
        .await?;
        Ok(())
    }
}
