use crate::guild_membership::*;
use crate::user::User;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct RemoveModForm {
    guild_tag: String,
    username: String,
}

#[post("/{guild_tag}/removemod/{username}")]
pub async fn handler(
    remove_moderator_form: web::Path<RemoveModForm>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    let formatted_form = RemoveModForm {
        guild_tag: remove_moderator_form.guild_tag.to_lowercase(),
        username: remove_moderator_form.username.to_lowercase(),
    };
    let valid_session = session_validation::policy_guild_admin(
        &session,
        &formatted_form.guild_tag,
        db_pool.get_ref(),
    )
    .await;
    match valid_session {
        Ok((None, Some(user))) => {
            //make sure user exists
            let user_exists =
                User::find_by_username_sensitive(&formatted_form.username, db_pool.get_ref()).await;
            match user_exists {
                Ok(Some(user)) => {
                    //make sure user is in guild
                    let user_is_member = GuildMembership::find_by_user_and_guild_tag(
                        &user.user_id,
                        &formatted_form.guild_tag,
                        db_pool.get_ref(),
                    )
                    .await;

                    match user_is_member {
                        Ok(Some(membership)) => match membership.is_moderator {
                            true => {
                                let mut tx = db_pool.begin().await.unwrap();
                                let updated_member = GuildMembership::update_membership_mod_status(
                                    false, membership, &mut tx,
                                )
                                .await;
                                match updated_member {
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
                                        return HttpResponse::Ok()
                                            .body("Mod removed successfully.");
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
                                        error!("Error removing user as mod: {}", err);
                                        return HttpResponse::InternalServerError()
                                            .body("Error removing user as mod.");
                                    }
                                }
                            }
                            false => {
                                return HttpResponse::BadRequest().body(
                                    "The user you are trying to remove as mod is not a mod.",
                                );
                            }
                        },
                        Ok(None) => {
                            return HttpResponse::BadRequest()
                            .body("The user you are trying to remove as mod is not a member of the guild.");
                        }
                        Err(err) => {
                            error!("Error fetching user guild membership: {}", err);
                            return HttpResponse::InternalServerError()
                                .body("Error fetching user.");
                        }
                    }
                }
                Ok(None) => {
                    return HttpResponse::NotFound()
                        .body("The user you are trying to remove as mod does not exist");
                }
                Err(err) => {
                    error!("Error fetching user: {}", err);
                    return HttpResponse::InternalServerError().body("Error fetching user.");
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
