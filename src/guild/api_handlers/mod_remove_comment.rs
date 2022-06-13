use crate::comment::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct RemoveCommentForm {
    guild_tag: String,
    comment_id: i32,
}

pub async fn handler(
    remove_comment_form: web::Json<RemoveCommentForm>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    let formatted_form = RemoveCommentForm {
        guild_tag: remove_comment_form.guild_tag.to_lowercase(),
        comment_id: remove_comment_form.comment_id,
    };
    let valid_session = session_validation::policy_guild_moderator_or_admin(
        &session,
        &formatted_form.guild_tag,
        db_pool.get_ref(),
    )
    .await;
    match valid_session {
        Ok((None, Some(user))) => {
            //make sure comment exists
            let comment_exists =
                Comment::find_by_comment_id(&formatted_form.comment_id, db_pool.get_ref()).await;
            match comment_exists {
                Ok(Some(comment)) => {
                    //delete comment
                    let mut tx = db_pool.begin().await.unwrap();
                    let deleted_comment = Comment::delete(&comment.comment_id, &mut tx).await;
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
                            return HttpResponse::Ok().body("Comment deleted.");
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
                        .body("The comment you are trying to remove does not exist.");
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
