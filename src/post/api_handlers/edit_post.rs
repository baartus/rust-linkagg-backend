use crate::post::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

//basically postform without options, is this even necessary?
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostEditRequestForm {
    pub post_id: i32,
    pub image_url: String,
    pub link_url: String,
    pub title: String,
    pub body: String,
}

pub async fn handler(
    post_edit_form: web::Json<PostEditRequestForm>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    //validate input (for now title must be less than 100 chars)
    //TODO: stricter input validation on this, make sure links are valid
    if post_edit_form.title.len() > 100 {
        return HttpResponse::BadRequest().body("Title must be less than 100 characters.");
    }
    if post_edit_form.title == "" {
        return HttpResponse::BadRequest().body("Title cannot be empty");
    }
    let valid_session = session_validation::policy_user(&session, db_pool.get_ref()).await;
    match valid_session {
        Ok((None, Some(user))) => {
            //make sure post exists and was made by user
            let existing_post =
                Post::find_by_post_id(&post_edit_form.post_id, db_pool.get_ref()).await;
            match existing_post {
                Ok(Some(post)) => {
                    if post.user_id != user.user_id {
                        return HttpResponse::Forbidden()
                            .body("You cannot edit someone else's post.");
                    }
                    //update post
                    //format link url and body
                    let formatted_link: Option<String>;
                    if post_edit_form.link_url == "" {
                        formatted_link = None;
                    } else {
                        formatted_link = Some(post_edit_form.link_url.clone());
                    }
                    let formatted_body: Option<String>;
                    if post_edit_form.body == "" {
                        formatted_body = None;
                    } else {
                        formatted_body = Some(post_edit_form.body.clone());
                    }
                    let formatted_image: Option<String>;
                    if post_edit_form.image_url == "" {
                        formatted_image = None;
                    } else {
                        formatted_image = Some(post_edit_form.image_url.clone());
                    }
                    //format edit form
                    let formatted_form = PostEditForm {
                        post_id: post_edit_form.post_id.clone(),
                        new_image_url: formatted_image,
                        new_link_url: formatted_link,
                        new_title: post_edit_form.title.clone(),
                        new_body: formatted_body,
                    };
                    let mut tx = db_pool.begin().await.unwrap();
                    let edited_post = Post::update(&formatted_form, &mut tx).await;
                    match edited_post {
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
                            return HttpResponse::Ok().body("Post edited successfully");
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
                            error!("Error editing post: {}", err);
                            return HttpResponse::InternalServerError().body("Error editing post.");
                        }
                    }
                }
                Ok(None) => {
                    return HttpResponse::BadRequest()
                        .body("The post you are trying to edit does not exist.");
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
