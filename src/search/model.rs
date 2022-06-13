use crate::comment::*;
use crate::guild::*;
use crate::post::*;
use crate::user::*;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Postgres, Row, Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Search {
    pub query: String,
    pub guild_tag: Option<String>,
}
