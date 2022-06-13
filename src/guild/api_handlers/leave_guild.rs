use crate::guild_membership::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

#[post("/leave/{guild_tag}")]
pub async fn handler(
    guild_tag: web::Path<String>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    let formatted_guild_tag = guild_tag.to_string().to_lowercase();
    let valid_session = session_validation::policy_user(&session, db_pool.get_ref()).await;
    match valid_session {
        Ok((None, Some(user))) => {
            //make sure user is in guild
            let existing_membership = GuildMembership::find_by_user_and_guild_tag(
                &user.user_id,
                &formatted_guild_tag,
                db_pool.get_ref(),
            )
            .await;
            match existing_membership {
                Ok(Some(membership)) => {
                    //make sure not admin
                    if membership.is_admin {
                        return HttpResponse::Forbidden()
                            .body("You cannot leave this guild as an admin.");
                    }
                    //leave guild
                    let mut tx = db_pool.begin().await.unwrap();
                    let deleted_membership = GuildMembership::delete(membership, &mut tx).await;
                    match deleted_membership {
                        Ok(()) => {
                            let succesful_commit = tx.commit().await;
                            match succesful_commit {
                                Ok(()) => (),
                                Err(err) => {
                                    error!("Error committing transaction: {}", err);
                                    return HttpResponse::InternalServerError()
                                        .body("Unknown Error.");
                                }
                            }
                            return HttpResponse::Ok().body("You have left the guild.");
                        }
                        Err(err) => {
                            let succesful_rollback = tx.rollback().await;
                            match succesful_rollback {
                                Ok(()) => (),
                                Err(err) => {
                                    error!("Error rolling back transaction: {}", err);
                                    return HttpResponse::InternalServerError()
                                        .body("Unknown Error.");
                                }
                            }
                            error!("Error deleting guild membership: {}", err);
                            return HttpResponse::InternalServerError()
                                .body("Error leaving guild.");
                        }
                    }
                }
                Err(err) => {
                    error!("Error validating session: {}", err);
                    return HttpResponse::InternalServerError()
                        .body("Error fetching guild membership.");
                }
                _ => {
                    return HttpResponse::BadRequest()
                        .body("You are not a member of this guild, or the guild does not exist.");
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
