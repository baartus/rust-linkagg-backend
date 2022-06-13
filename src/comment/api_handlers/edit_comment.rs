use crate::comment::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

//i still dont know if this pattern is necessary...
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditCommentForm {
    pub comment_id: i32,
    pub new_body: String,
}

pub async fn handler(
    edit_form: web::Json<EditCommentForm>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    let valid_session = session_validation::policy_user(&session, db_pool.get_ref()).await;
    match valid_session {
        Ok((None, Some(user))) => {
            //make sure comment exists and user is owner
            let existing_comment =
                Comment::find_by_comment_id(&edit_form.comment_id, db_pool.get_ref()).await;
            match existing_comment {
                Ok(Some(comment)) => {
                    if user.user_id != comment.user_id {
                        return HttpResponse::Forbidden()
                            .body("You cannot edit someone else's comment.");
                    }
                    if edit_form.new_body == "" {
                        return HttpResponse::BadRequest().body("Comment cannot be empty");
                    }
                    //update comment
                    let mut tx = db_pool.begin().await.unwrap();
                    let edited_comment =
                        Comment::update(&edit_form.comment_id, &edit_form.new_body, &mut tx).await;
                    match edited_comment {
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
                            return HttpResponse::Ok().body("Comment edited successfully");
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
                            error!("Error editing comment: {}", err);
                            return HttpResponse::InternalServerError()
                                .body("Error editing comment.");
                        }
                    }
                }
                Ok(None) => {
                    return HttpResponse::BadRequest()
                        .body("The comment you are trying to delete does not exist.");
                }
                Err(err) => {
                    error!("Error fetching comment: {}", err);
                    return HttpResponse::InternalServerError().body("Error fetching comment.");
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
