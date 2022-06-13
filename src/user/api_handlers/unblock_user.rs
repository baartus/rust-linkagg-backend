use crate::block::*;
use crate::user::User;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

#[post("/unblock/{username}")]
pub async fn handler(
    username: web::Path<String>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    let formatted_username = username.to_string().to_lowercase();
    let valid_session = session_validation::policy_user(&session, db_pool.get_ref()).await;
    match valid_session {
        Ok((None, Some(user))) => {
            //make sure user to unblock exists
            let existing_user =
                User::find_by_username_sensitive(&formatted_username, db_pool.get_ref()).await;
            match existing_user {
                Ok(Some(blocked_user)) => {
                    //check if user already has person blocked
                    let existing_block = Block::find_by_user_and_blocked_user_username(
                        &user.user_id,
                        &formatted_username,
                        db_pool.get_ref(),
                    )
                    .await;
                    match existing_block {
                        Ok(Some(block)) => (),
                        Ok(None) => {
                            return HttpResponse::BadRequest()
                                .body("You don't have this user blocked");
                        }
                        Err(err) => {
                            error!("Error fetching block list: {}", err);
                            return HttpResponse::InternalServerError()
                                .body("Error fetching block list.");
                        }
                    }
                    //delete block
                    let mut tx = db_pool.begin().await.unwrap();
                    let deleted_block =
                        Block::delete(&user.user_id, &formatted_username, &mut tx).await;
                    match deleted_block {
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
                            return HttpResponse::Ok().body("User successfully unblocked.");
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
                            error!("Error unblocking user: {}", err);
                            return HttpResponse::InternalServerError()
                                .body("Error unblocking user.");
                        }
                    }
                }
                Ok(None) => {
                    return HttpResponse::BadRequest().body("You already have this user blocked");
                }
                Err(err) => {
                    error!("Error fetching block list: {}", err);
                    return HttpResponse::InternalServerError().body("Error fetching block list.");
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
