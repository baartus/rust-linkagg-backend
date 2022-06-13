use crate::post::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

//basically postform without options, is this even necessary?
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostLockForm {
    pub post_id: i32,
}

pub async fn handler(
    post_lock_form: web::Json<PostLockForm>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    let existing_post = Post::find_by_post_id(&post_lock_form.post_id, db_pool.get_ref()).await;
    match existing_post {
        Ok(Some(post)) => {
            let valid_session = session_validation::policy_guild_moderator_or_admin(
                &session,
                &post.guild_tag,
                db_pool.get_ref(),
            )
            .await;
            match valid_session {
                Ok((None, Some(user))) => {
                    //lock post
                    let mut tx = db_pool.begin().await.unwrap();
                    let locked_post =
                        Post::update_lock(&post_lock_form.post_id, true, &mut tx).await;
                    match locked_post {
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
                            return HttpResponse::Ok().body("Post locked successfully");
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
                            error!("Error locking post: {}", err);
                            return HttpResponse::InternalServerError().body("Error locking post.");
                        }
                    }
                }
                Ok((Some(response), None)) => {
                    return response;
                }
                Err(err) => {
                    error!("Error verifying user session: {}", err);
                    return HttpResponse::InternalServerError()
                        .body("Error verifying user session.");
                }
                _ => {
                    return HttpResponse::InternalServerError().body("Unknown Error.");
                }
            }
        }
        Ok(None) => {
            return HttpResponse::BadRequest()
                .body("The post you are trying to lock does not exist.");
        }
        Err(err) => {
            error!("Error fetching post: {}", err);
            return HttpResponse::InternalServerError().body("Error fetching post.");
        }
    }
}
