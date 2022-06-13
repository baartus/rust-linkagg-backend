use crate::utils::session_validation;
use crate::view::DetailedGuildView;
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailedGuildViewResponse {
    is_member: bool,
    guild_details: DetailedGuildView,
}

#[get("/guild/{guild_tag}")]
pub async fn handler(
    db_pool: web::Data<PgPool>,
    guild_tag: web::Path<String>,
    session: Session,
) -> impl Responder {
    let formatted_tag = guild_tag.to_string().to_lowercase();
    let is_guild_member =
        session_validation::policy_guild_member(&session, &formatted_tag, db_pool.get_ref()).await;
    match is_guild_member {
        Ok((None, Some(user))) => {
            //user is guild member
            let detailed_guild_view =
                DetailedGuildView::find_by_guild_tag(&formatted_tag, db_pool.get_ref()).await;
            match detailed_guild_view {
                Ok(Some(guild)) => {
                    let resp = DetailedGuildViewResponse {
                        is_member: true,
                        guild_details: guild,
                    };
                    HttpResponse::Ok().json(resp)
                }
                Ok(None) => return HttpResponse::BadRequest().body("Guild does not exist."),
                Err(err) => {
                    error!("Error fetching guild: {}", err);
                    return HttpResponse::InternalServerError().body("Error fetching guild.");
                }
            }
        }
        Ok((Some(response), None)) => {
            //user is not guild member
            let detailed_guild_view =
                DetailedGuildView::find_by_guild_tag(&formatted_tag, db_pool.get_ref()).await;
            match detailed_guild_view {
                Ok(Some(guild)) => {
                    let resp = DetailedGuildViewResponse {
                        is_member: false,
                        guild_details: guild,
                    };
                    HttpResponse::Ok().json(resp)
                }
                Ok(None) => return HttpResponse::BadRequest().body("Guild does not exist."),
                Err(err) => {
                    error!("Error fetching guild: {}", err);
                    return HttpResponse::InternalServerError().body("Error fetching guild.");
                }
            }
        }
        Err(err) => {
            error!("Error verifying user session: {}", err);
            return HttpResponse::InternalServerError().body("Error verifying user session.");
        }
        _ => {
            return HttpResponse::InternalServerError().body("Unknown Error.");
        }
    }
}
