use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Site {
    pub description: Option<String>,
}
impl Site {
    pub async fn update_description(
        new_description: &String,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE site_info
            SET description = $1
            "#,
            new_description
        )
        .execute(tx)
        .await?;

        Ok(())
    }
}
