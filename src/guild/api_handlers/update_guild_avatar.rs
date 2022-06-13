use crate::guild::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateGuildAvatarForm {
    avatar_url: String,
}

#[post("/{guild_tag}/mod/updateavatar")]
pub async fn handler(
    guild_tag: web::Path<String>,
    update_form: web::Json<UpdateGuildAvatarForm>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    let formatted_tag = guild_tag.to_string().to_lowercase();
    let formatted_avatar_url: Option<String>;
    if update_form.avatar_url == "" {
        formatted_avatar_url = None;
    } else {
        formatted_avatar_url = Some(update_form.avatar_url.clone());
    }
    let valid_session = session_validation::policy_guild_moderator_or_admin(
        &session,
        &formatted_tag,
        db_pool.get_ref(),
    )
    .await;
    match valid_session {
        Ok((None, Some(user))) => {
            //update avatar
            let mut tx = db_pool.begin().await.unwrap();
            let updated_avatar =
                Guild::update_guild_description(&formatted_avatar_url, &formatted_tag, &mut tx)
                    .await;
            match updated_avatar {
                Ok(()) => {
                    let succesful_commit = tx.commit().await;
                    match succesful_commit {
                        Ok(()) => (),
                        Err(err) => {
                            error!("Error committing transaction: {}", err);
                            return HttpResponse::InternalServerError().body("Unknown Error.");
                        }
                    }
                    return HttpResponse::Ok().body("Guild avatar updated successfully.");
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
                    error!("Error updating guild avatar: {}", err);
                    return HttpResponse::InternalServerError()
                        .body("Error updating guild avatar.");
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
