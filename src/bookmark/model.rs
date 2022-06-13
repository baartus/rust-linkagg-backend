use serde::{Deserialize, Serialize};
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bookmark {
    pub bookmark_id: i32
    pub post_id: i32,
    pub user_id: i32,
}
pub struct BookmarkForm {
    post_id: i32,
    user_id: i32,
}

impl Bookmark {
    pub async fn create(bookmark_form: BookmarkForm, pool: &PgPool) -> Result<()> {}
    pub async fn find_by_bookmark_id(bookmark_id: i32, pool: &PgPool) -> Result<Option<Bookmark>> {}
    pub async fn find_bookmarks_by_user_id(user_id: i32, pool: &PgPool) -> Result<Option<Vec<Bookmark>>> {}
    pub async fn delete(bookmark: Bookmark, pool: &PgPool) -> Result<()> {}
}