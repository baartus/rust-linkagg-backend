use crate::guild::*;
use crate::post::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

//basically postform without user id, probably move this?
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePostForm {
    pub guild_tag: String,
    pub image_url: String,
    pub link_url: String,
    pub title: String,
    pub body: String,
}

pub async fn handler(
    post_form: web::Json<CreatePostForm>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    //validate input (for now title must be less than 100 chars)
    //TODO: stricter input validation on this, make sure links are valid
    if post_form.title.len() > 100 {
        return HttpResponse::BadRequest().body("Title must be less than 100 characters.");
    }
    if post_form.title == "" {
        return HttpResponse::BadRequest().body("Title cannot be empty");
    }

    //TODO: posts with out a title should not be accepted, investigate

    let valid_session =
        session_validation::policy_guild_member(&session, &post_form.guild_tag, db_pool.get_ref())
            .await;
    match valid_session {
        Ok((None, Some(user))) => {
            //format link url and body
            let formatted_link: Option<String>;
            if post_form.link_url == "" {
                formatted_link = None;
            } else {
                formatted_link = Some(post_form.link_url.clone());
            }
            let formatted_image: Option<String>;
            if post_form.image_url == "" {
                formatted_image = None;
            } else {
                formatted_image = Some(post_form.image_url.clone());
            }
            let formatted_body: Option<String>;
            if post_form.body == "" {
                formatted_body = None;
            } else {
                formatted_body = Some(post_form.body.clone());
            }
            //format form
            let formatted_form = PostForm {
                guild_tag: post_form.guild_tag.clone().to_lowercase(),
                user_id: user.user_id,
                image_url: formatted_image,
                link_url: formatted_link,
                title: post_form.title.clone(),
                body: formatted_body,
            };
            //make sure guild exists (this logic can probably be removed now that i added the guild member policy)
            let existing_guild =
                Guild::find_by_guild_tag(&formatted_form.guild_tag, db_pool.get_ref()).await;
            match existing_guild {
                Ok(Some(guild)) => {
                    //make sure user is guild member, and isn't banned.

                    //create post
                    let mut tx = db_pool.begin().await.unwrap();
                    let created_post = Post::create(&formatted_form, &mut tx).await;
                    match created_post {
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
                            return HttpResponse::Ok().body("Post created successfully");
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
                            error!("Error creating post: {}", err);
                            return HttpResponse::InternalServerError()
                                .body("Error creating post.");
                        }
                    }
                }
                Err(err) => {
                    error!("Error checking guilds: {}", err);
                    return HttpResponse::InternalServerError().body("Error creating post.");
                }
                _ => {
                    return HttpResponse::BadRequest()
                        .body("Error creating post: a guild with that tag does not exist.");
                }
            }
            //create post
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
