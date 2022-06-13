use crate::guild::*;
use crate::guild_membership::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn handler(
    guild_form: web::Json<GuildForm>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    //make sure user has a valid session
    //guild tag validation, must be all alphanumeric, 15 chars max
    let mut valid_guild_tag: bool;
    valid_guild_tag = guild_form.guild_tag.chars().all(char::is_alphanumeric);
    if guild_form.guild_tag.len() > 15 {
        valid_guild_tag = false;
    }
    if !valid_guild_tag {
        return HttpResponse::NotAcceptable()
                .body("Guild tags can only contain alphanumeric characters, and must be less than 15 characters");
    }
    //guild name validation, 15 chars max
    let mut valid_guild_name = true;
    if guild_form.guild_name.len() > 25 {
        valid_guild_name = false;
    }
    if !valid_guild_name {
        return HttpResponse::NotAcceptable()
            .body("Guild names must be no longer than 25 characters");
    }

    let cloned_form = guild_form.clone();

    let formatted_form = GuildForm {
        guild_tag: cloned_form.guild_tag.to_lowercase(),
        guild_name: cloned_form.guild_name,
    };

    let valid_session = session_validation::policy_admin(&session, db_pool.get_ref()).await;
    match valid_session {
        Ok((None, Some(user))) => {
            //make sure guild doesn't already exist
            let existing_guild =
                Guild::find_by_guild_tag(&formatted_form.guild_tag, db_pool.get_ref()).await;
            match existing_guild {
                Ok(Some(guildy)) => {
                    return HttpResponse::InternalServerError()
                        .body("Error creating guild: a guild with that tag already exists.");
                }
                Err(err) => {
                    error!("Error checking guilds: {}", err);
                    return HttpResponse::InternalServerError().body("Error creating guild.");
                }
                _ => (),
            }
            //create guild
            let mut tx = db_pool.begin().await.unwrap();
            let make_guild = Guild::create(&formatted_form, &mut tx).await;
            match make_guild {
                Ok(()) => {
                    let membership_form = GuildMembershipForm {
                        user_id: user.user_id,
                        guild_tag: formatted_form.clone().guild_tag,
                    };
                    let created_admin =
                        GuildMembership::create_as_admin(&membership_form, &mut tx).await;
                    match created_admin {
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
                            return HttpResponse::Ok().body("Guild sucessfully created");
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
                            error!("Error creating guild admin: {}", err);
                            return HttpResponse::InternalServerError()
                                .body("Error creating guild.");
                        }
                    }
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
                    error!("Error creating guild: {}", err);
                    return HttpResponse::InternalServerError().body("Error creating guild.");
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
