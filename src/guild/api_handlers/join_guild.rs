use crate::guild::*;
use crate::guild_membership::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

#[post("/join/{guild_tag}")]
pub async fn handler(
    guild_tag: web::Path<String>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    let formatted_guild_tag = guild_tag.to_string().to_lowercase();
    let valid_session = session_validation::policy_user(&session, db_pool.get_ref()).await;
    match valid_session {
        Ok((None, Some(user))) => {
            //make sure user isn't already in the guild
            let existing_membership = GuildMembership::find_by_user_and_guild_tag(
                &user.user_id,
                &formatted_guild_tag,
                db_pool.get_ref(),
            )
            .await;

            match existing_membership {
                Ok(Some(membership)) => {
                    return HttpResponse::BadRequest()
                        .body("You are already a member of this guild.");
                }
                Err(err) => {
                    error!("Error validating session: {}", err);
                    return HttpResponse::InternalServerError()
                        .body("Error fetching guild membership.");
                }
                _ => (),
            }
            //make sure guild exists
            let existing_guild =
                Guild::find_by_guild_tag(&formatted_guild_tag, db_pool.get_ref()).await;
            match existing_guild {
                Ok(Some(guild)) => {
                    //create guild membership
                    let mut tx = db_pool.begin().await.unwrap();
                    //update guild aggs
                    let guild_membership_form = GuildMembershipForm {
                        user_id: user.user_id,
                        guild_tag: formatted_guild_tag.clone(),
                    };
                    let new_membership =
                        GuildMembership::create(&guild_membership_form, &mut tx).await;
                    match new_membership {
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
                            return HttpResponse::Ok().body("Guild joined sucessfully.");
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
                            error!("Error creating guild membership: {}", err);
                            return HttpResponse::InternalServerError()
                                .body("Error creating guild membership.");
                        }
                    }
                }
                Ok(None) => {
                    return HttpResponse::NotFound()
                        .body("The guild you are trying to join does not exist");
                }
                Err(err) => {
                    error!("Error creating guild membership: {}", err);
                    return HttpResponse::InternalServerError().body("Error fetching guild data.");
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
