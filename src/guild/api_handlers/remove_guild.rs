use crate::guild::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

#[post("/remove/{guild_tag}")]
pub async fn handler(
    guild_tag: web::Path<String>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    let formatted_guild_tag = guild_tag.to_string().to_lowercase();
    let valid_session =
        session_validation::policy_guild_admin(&session, &formatted_guild_tag, db_pool.get_ref())
            .await;
    match valid_session {
        Ok((None, Some(user))) => {
            //delete
            let mut tx = db_pool.begin().await.unwrap();
            let deleted = Guild::delete(&formatted_guild_tag, &mut tx).await;
            match deleted {
                Ok(()) => {
                    let succesful_commit = tx.commit().await;
                    match succesful_commit {
                        Ok(()) => (),
                        Err(err) => {
                            error!("Error committing transaction: {}", err);
                            return HttpResponse::InternalServerError().body("Unknown Error.");
                        }
                    }
                    return HttpResponse::Ok().body("Guild removed sucessfully.");
                }
                Err(err) => {
                    let succesful_rollback = tx.rollback().await;
                    match succesful_rollback {
                        Ok(()) => (),
                        Err(err) => {
                            error!("Error rolling back transaction: {}", err);
                            return HttpResponse::InternalServerError().body("Unknown Error.");
                        }
                    }
                    error!("Error verifying user session: {}", err);
                    return HttpResponse::InternalServerError().body("Error deleting guild.");
                }
            }
        }
        Ok((Some(response), None)) => {
            return response;
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
