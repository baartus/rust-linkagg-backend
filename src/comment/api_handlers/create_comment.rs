use crate::comment::*;
use crate::post::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

//i still dont know if this pattern is necessary...
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCommentForm {
    pub post_id: i32,
    pub parent_comment_id: i32,
    pub body: String,
}

pub async fn handler(
    comment_form: web::Json<CreateCommentForm>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    let valid_session = session_validation::policy_user(&session, db_pool.get_ref()).await;
    match valid_session {
        Ok((None, Some(user))) => {
            //make sure post exists
            let existing_post =
                Post::find_by_post_id(&comment_form.post_id, db_pool.get_ref()).await;
            match existing_post {
                Ok(Some(post)) => {
                    //make sure post isn't locked
                    if post.is_locked {
                        return HttpResponse::Forbidden()
                            .body("The post you are trying to comment on is locked.");
                    }
                    //format comment properly
                    if comment_form.body == "" {
                        return HttpResponse::BadRequest().body("Comment cannot be empty");
                    }
                    let formatted_parent_id: Option<i32>;
                    if comment_form.parent_comment_id == 0 {
                        formatted_parent_id = None;
                    } else {
                        formatted_parent_id = Some(comment_form.parent_comment_id);
                    }
                    let formatted_comment_form = CommentForm {
                        post_id: comment_form.post_id,
                        parent_comment_id: formatted_parent_id,
                        user_id: user.user_id,
                        body: comment_form.body.clone(),
                    };
                    //create comment
                    let mut tx = db_pool.begin().await.unwrap();
                    let created_comment = Comment::create(&formatted_comment_form, &mut tx).await;
                    match created_comment {
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
                            return HttpResponse::Ok().body("Comment created successfully");
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
                            error!("Error creating comment: {}", err);
                            return HttpResponse::InternalServerError()
                                .body("Error creating comment.");
                        }
                    }
                }
                Ok(None) => {
                    return HttpResponse::BadRequest()
                        .body("The post you are trying to comment on does not exist.");
                }
                Err(err) => {
                    error!("Error verifying user session: {}", err);
                    return HttpResponse::InternalServerError().body("Error fetching post.");
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
