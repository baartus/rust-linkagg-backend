use crate::guild_membership::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct BanUserForm {
    guild_tag: String,
    username: String,
}

#[post("/{guild_tag}/ban/{username}")]
pub async fn handler(
    appoint_moderator_form: web::Path<BanUserForm>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    let formatted_form = BanUserForm {
        guild_tag: appoint_moderator_form.guild_tag.to_lowercase(),
        username: appoint_moderator_form.username.to_lowercase(),
    };
    let valid_session = session_validation::policy_guild_moderator_or_admin(
        &session,
        &formatted_form.guild_tag,
        db_pool.get_ref(),
    )
    .await;
    match valid_session {
        Ok((None, Some(user))) => {
            //make sure user is in guild
            let user_is_member = GuildMembership::find_by_user_and_guild_tag(
                &user.user_id,
                &formatted_form.guild_tag,
                db_pool.get_ref(),
            )
            .await;

            match user_is_member {
                Ok(Some(membership)) => {
                    //update membership with ban
                    //make sure not already mod
                    if membership.is_moderator || membership.is_admin {
                        return HttpResponse::BadRequest()
                            .body("The user you are trying to ban is already a mod or admin.");
                    }
                    if membership.is_banned {
                        return HttpResponse::BadRequest()
                            .body("The user you are trying to ban is already banned.");
                    }
                    //update membership
                    let mut tx = db_pool.begin().await.unwrap();
                    let updated_member =
                        GuildMembership::update_membership_ban_status(true, membership, &mut tx)
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
                            return HttpResponse::Ok().body("User has been banned from the guild.");
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
                            error!("Error banning user: {}", err);
                            return HttpResponse::InternalServerError().body("Error banning user");
                        }
                    }
                }
                Ok(None) => {
                    return HttpResponse::BadRequest()
                        .body("The user you are trying to ban is not in the guild.");
                }
                Err(err) => {
                    error!("Error fetching user guild membership: {}", err);
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
