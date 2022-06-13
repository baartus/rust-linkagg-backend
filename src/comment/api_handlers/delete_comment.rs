use crate::comment::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

//i still dont know if this pattern is necessary...
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteCommentForm {
    pub comment_id: i32,
}

pub async fn handler(
    delete_form: web::Json<DeleteCommentForm>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    let valid_session = session_validation::policy_user(&session, db_pool.get_ref()).await;
    match valid_session {
        Ok((None, Some(user))) => {
            //make sure comment exists and user is owner
            let existing_comment =
                Comment::find_by_comment_id(&delete_form.comment_id, db_pool.get_ref()).await;
            match existing_comment {
                Ok(Some(comment)) => {
                    if user.user_id != comment.user_id {
                        return HttpResponse::Forbidden()
                            .body("You cannot delete someone else's comment.");
                    }
                    let mut tx = db_pool.begin().await.unwrap();
                    //delete comment
                    let deleted_comment = Comment::delete(&delete_form.comment_id, &mut tx).await;
                    match deleted_comment {
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
                            return HttpResponse::Ok().body("Comment deleted successfully");
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
                            error!("Error deleting comment: {}", err);
                            return HttpResponse::InternalServerError()
                                .body("Error deleting comment.");
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
