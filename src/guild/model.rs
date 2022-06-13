use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Guild {
    pub guild_tag: String,
    pub guild_name: String,
    pub guild_description: Option<String>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub is_banned: bool,
    pub created_at: String, //convert time to string
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildForm {
    pub guild_tag: String,
    pub guild_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildJoinForm {
    pub guild_tag: String,
}

impl Guild {
    pub async fn create(guild_form: &GuildForm, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO guilds (guild_tag, guild_name)
            VALUES ($1, $2)
            "#,
            guild_form.guild_tag,
            guild_form.guild_name
        )
        .execute(tx)
        .await?;
        Ok(())
    }
    pub async fn find_by_guild_tag(guild_tag: &String, pool: &PgPool) -> Result<Option<Guild>> {
        let guild = sqlx::query!(
            r#"
            SELECT * FROM guilds
            WHERE guild_tag = $1
            "#,
            guild_tag
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(guild.map(|guild| Guild {
            guild_tag: guild.guild_tag,
            guild_name: guild.guild_name,
            guild_description: guild.guild_description,
            avatar_url: guild.avatar_url,
            banner_url: guild.banner_url,
            is_banned: guild.is_banned,
            created_at: guild.created_at.to_string(), //convert time to string
        }))
    }

    //find all guilds, ordered by number of members in each guild
    pub async fn find_all(
        pool: &PgPool,
        results_per_page: &i64,
        page_number: &i64,
    ) -> Result<Vec<Guild>> {
        let guilds = sqlx::query!(
            r#"
            SELECT guilds.*, (SELECT COUNT(*) FROM guild_memberships WHERE guild_memberships.guild_tag = guilds.guild_tag) AS members FROM guilds
            ORDER BY members
            LIMIT $1
            OFFSET $2
            "#,
            results_per_page,
            ((page_number - 1) * results_per_page)
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|guild| Guild {
            guild_tag: guild.guild_tag,
            guild_name: guild.guild_name,
            guild_description: guild.guild_description,
            avatar_url: guild.avatar_url,
            banner_url: guild.banner_url,
            is_banned: guild.is_banned,
            created_at: guild.created_at.to_string(), //convert time to string
        })
        .collect();
        Ok(guilds)
    }

    //update name
    pub async fn update_guild_name(
        new_guild_name: String,
        guild_tag: &String,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE guilds
            SET guild_name = $2
            WHERE guild_tag = $1
            "#,
            guild_tag,
            new_guild_name
        )
        .execute(tx)
        .await?;

        Ok(())
    }
    /*TODO: probably need to refactor a ton of this code to add guild ids, and allow for changing guild tags... really stupid decision I made here... not sure why..
    :(
    //update url
    pub async fn update_guild_tag(
        new_guild_tag: &String,
        old_guild: Guild,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE guilds
            SET guild_tag = $2
            WHERE guild_tag = $1
            "#,
            old_guild.guild_tag,
            new_guild_tag
        )
        .execute(tx)
        .await?;

        Ok(())
    }
    */
    //update description
    pub async fn update_guild_description(
        new_guild_description: &Option<String>,
        guild_tag: &String,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        match new_guild_description {
            Some(description) => {
                sqlx::query!(
                    r#"
                    UPDATE guilds
                    SET guild_description = $2
                    WHERE guild_tag = $1
                    "#,
                    guild_tag,
                    description
                )
                .execute(tx)
                .await?;
            }
            None => {
                sqlx::query!(
                    r#"
                    UPDATE guilds
                    SET guild_description = NULL
                    WHERE guild_tag = $1
                    "#,
                    guild_tag,
                )
                .execute(tx)
                .await?;
            }
        }

        Ok(())
    }
    //update avatar url
    pub async fn update_guild_avatar(
        new_avatar_url: &Option<String>,
        guild_tag: &String,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        match new_avatar_url {
            Some(avatar_url) => {
                sqlx::query!(
                    r#"
                    UPDATE guilds
                    SET avatar_url = $2
                    WHERE guild_tag = $1
                    "#,
                    guild_tag,
                    avatar_url
                )
                .execute(tx)
                .await?;
            }
            None => {
                sqlx::query!(
                    r#"
                    UPDATE guilds
                    SET avatar_url = NULL
                    WHERE guild_tag = $1
                    "#,
                    guild_tag,
                )
                .execute(tx)
                .await?;
            }
        }

        Ok(())
    }
    //update banner url
    pub async fn update_guild_banner(
        new_banner_url: &Option<String>,
        guild_tag: &String,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        match new_banner_url {
            Some(banner_url) => {
                sqlx::query!(
                    r#"
                    UPDATE guilds
                    SET banner_url = $2
                    WHERE guild_tag = $1
                    "#,
                    guild_tag,
                    banner_url
                )
                .execute(tx)
                .await?;
            }
            None => {
                sqlx::query!(
                    r#"
                    UPDATE guilds
                    SET banner_url = NULL
                    WHERE guild_tag = $1
                    "#,
                    guild_tag,
                )
                .execute(tx)
                .await?;
            }
        }

        Ok(())
    }
    //update banned status
    pub async fn update_guild_ban_status(
        new_ban_status: bool,
        guild_tag: &String,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE guilds
            SET is_banned = $2
            WHERE guild_tag = $1
            "#,
            guild_tag,
            new_ban_status
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn delete(guild_tag: &String, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM guilds
            where guild_tag = $1
            "#,
            guild_tag
        )
        .execute(tx)
        .await?;
        Ok(())
    }
}
