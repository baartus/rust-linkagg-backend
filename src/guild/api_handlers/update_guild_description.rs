use crate::guild::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateGuildDescriptionForm {
    description: String,
}

#[post("/{guild_tag}/mod/updatedescription")]
pub async fn handler(
    guild_tag: web::Path<String>,
    update_form: web::Json<UpdateGuildDescriptionForm>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    let formatted_tag = guild_tag.to_string().to_lowercase();
    let formatted_description: Option<String>;
    if update_form.description == "" {
        formatted_description = None;
    } else {
        formatted_description = Some(update_form.description.clone());
    }
    let valid_session = session_validation::policy_guild_moderator_or_admin(
        &session,
        &formatted_tag,
        db_pool.get_ref(),
    )
    .await;
    match valid_session {
        Ok((None, Some(user))) => {
            //update description
            let mut tx = db_pool.begin().await.unwrap();
            let updated_description =
                Guild::update_guild_description(&formatted_description, &formatted_tag, &mut tx)
                    .await;
            match updated_description {
                Ok(()) => {
                    let succesful_commit = tx.commit().await;
                    match succesful_commit {
                        Ok(()) => (),
                        Err(err) => {
                            error!("Error committing transaction: {}", err);
                            return HttpResponse::InternalServerError().body("Unknown Error.");
                        }
                    }
                    return HttpResponse::Ok().body("Guild description updated successfully.");
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
                    error!("Error updating guild description: {}", err);
                    return HttpResponse::InternalServerError()
                        .body("Error updating guild description.");
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
