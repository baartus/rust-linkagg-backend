use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Report {
    pub report_id: i32,
    pub post_id: Option<i32>,
    pub comment_id: Option<i32>,
    pub reason: String,
    pub addressed: bool,
    pub created_at: String, //convert time to string
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportForm {
    pub post_id: i32,
    pub comment_id: i32,
    pub reason: String,
}

//todo: auto-dele
impl Report {
    pub async fn find_all(
        results_per_page: &i64,
        page_number: &i64,
        pool: &PgPool,
    ) -> Result<Vec<Report>> {
        let reports = sqlx::query!(
            r#"
            SELECT * FROM reports
            ORDER BY created_at DESC
            LIMIT $1
            OFFSET $2
            "#,
            results_per_page,
            ((page_number - 1) * results_per_page)
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|report| Report {
            report_id: report.report_id,
            post_id: report.post_id,
            comment_id: report.comment_id,
            reason: report.reason,
            addressed: report.addressed,
            created_at: report.created_at.to_string(),
        })
        .collect();
        Ok(reports)
    }
    pub async fn find_all_by_post_id(
        post_id: &i32,
        results_per_page: &i64,
        page_number: &i64,
        pool: &PgPool,
    ) -> Result<Vec<Report>> {
        let reports = sqlx::query!(
            r#"
            SELECT * FROM reports
            WHERE post_id = $1
            LIMIT $2
            OFFSET $3
            "#,
            post_id,
            results_per_page,
            ((page_number - 1) * results_per_page)
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|report| Report {
            report_id: report.report_id,
            post_id: report.post_id,
            comment_id: report.comment_id,
            reason: report.reason,
            addressed: report.addressed,
            created_at: report.created_at.to_string(),
        })
        .collect();
        Ok(reports)
    }
    pub async fn find_by_report_id(report_id: &i32, pool: &PgPool) -> Result<Option<Report>> {
        let report = sqlx::query!(
            r#"
            SELECT * FROM reports
            WHERE report_id = $1
            "#,
            report_id
        )
        .fetch_optional(&*pool)
        .await?;
        Ok(report.map(|report| Report {
            report_id: report.report_id,
            post_id: report.post_id,
            comment_id: report.comment_id,
            reason: report.reason,
            addressed: report.addressed,
            created_at: report.created_at.to_string(),
        }))
    }
    pub async fn create(
        report_form: &ReportForm,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        if report_form.comment_id == 0 {
            sqlx::query!(
                r#"
                INSERT INTO reports (post_id, reason)
                VALUES ($1, $2)
                "#,
                report_form.post_id,
                report_form.reason
            )
            .execute(tx)
            .await?;
        } else if report_form.post_id == 0 {
            sqlx::query!(
                r#"
                INSERT INTO reports (comment_id, reason)
                VALUES ($1, $2)
                "#,
                report_form.comment_id,
                report_form.reason
            )
            .execute(tx)
            .await?;
        }
        Ok(())
    }
    pub async fn update_addressed_status(
        report_id: &i32,
        addressed: bool,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE reports
            SET addressed = $2
            WHERE report_id = $1
            "#,
            report_id,
            addressed
        )
        .execute(tx)
        .await?;
        Ok(())
    }
}
