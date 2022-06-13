use crate::post::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct RemovePostForm {
    guild_tag: String,
    post_id: i32,
}

pub async fn handler(
    remove_post_form: web::Json<RemovePostForm>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    let formatted_form = RemovePostForm {
        guild_tag: remove_post_form.guild_tag.to_lowercase(),
        post_id: remove_post_form.post_id,
    };
    let valid_session = session_validation::policy_guild_moderator_or_admin(
        &session,
        &formatted_form.guild_tag,
        db_pool.get_ref(),
    )
    .await;
    match valid_session {
        Ok((None, Some(user))) => {
            //make sure post exists
            let post_exists =
                Post::find_by_post_id(&formatted_form.post_id, db_pool.get_ref()).await;
            match post_exists {
                Ok(Some(post)) => {
                    //delete post
                    let mut tx = db_pool.begin().await.unwrap();
                    let deleted_post = Post::delete(&post.post_id, &mut tx).await;
                    match deleted_post {
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
                            return HttpResponse::Ok().body("Post deleted.");
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
                            error!("Error deleting post: {}", err);
                            return HttpResponse::InternalServerError()
                                .body("Error deleting post.");
                        }
                    }
                }
                Ok(None) => {
                    return HttpResponse::BadRequest()
                        .body("The post you are trying to remove does not exist.");
                }
                Err(err) => {
                    error!("Error fetching post: {}", err);
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
