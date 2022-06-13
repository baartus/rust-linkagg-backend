use crate::comment::*;
use crate::comment_vote::*;
use crate::post::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

//TODO: move voting aggregate shit into SQL triggers

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpvoteCommentForm {
    comment_id: i32,
    up: bool,
}

pub async fn handler(
    vote_form: web::Json<UpvoteCommentForm>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    //make sure comment exists
    let comment_exists =
        Comment::find_by_comment_id(&vote_form.comment_id, db_pool.get_ref()).await;
    match comment_exists {
        Ok(Some(comment)) => {
            //get comments parent post to check if locked
            let post_exists = Post::find_by_post_id(&comment.post_id, db_pool.get_ref()).await;
            match post_exists {
                Ok(Some(post)) => {
                    //make sure post isn't locked
                    if post.is_locked {
                        return HttpResponse::Forbidden()
                            .body("The post you are trying to upvote is locked.");
                    }
                }
                Err(err) => {
                    error!("Error fetching post: {}", err);
                    return HttpResponse::InternalServerError().body("Error fetching post.");
                }
                _ => (),
            }
        }
        Ok(None) => {
            return HttpResponse::BadRequest()
                .body("The comment you are trying to upvote does not exist.");
        }
        Err(err) => {
            error!("Error fetching comment: {}", err);
            return HttpResponse::InternalServerError().body("Error fetching comment.");
        }
    }

    let valid_session = session_validation::policy_user(&session, db_pool.get_ref()).await;
    match valid_session {
        Ok((None, Some(user))) => {
            //make sure user hasn't already upvoted comment
            let existing_vote = CommentVote::find_by_comment_and_user_id(
                &vote_form.comment_id,
                &user.user_id,
                db_pool.get_ref(),
            )
            .await;
            match existing_vote {
                Ok(Some(vote)) => {
                    //if votes are the same delete, otherwise update
                    if vote.up == vote_form.up {
                        //delete vote
                        let mut tx = db_pool.begin().await.unwrap();
                        let deleted_vote =
                            CommentVote::delete(&vote_form.comment_id, &user.user_id, &mut tx)
                                .await;
                        match deleted_vote {
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
                                return HttpResponse::Ok().body("Vote successfully undone");
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
                                error!("Error deleting vote: {}", err);
                                return HttpResponse::InternalServerError()
                                    .body("Error undoing vote.");
                            }
                        }
                    } else {
                        //update vote
                        let mut tx = db_pool.begin().await.unwrap();
                        let formatted_form = CommentVote {
                            comment_id: vote_form.comment_id,
                            user_id: user.user_id,
                            up: vote_form.up,
                        };
                        let updated_vote = CommentVote::update(&formatted_form, &mut tx).await;
                        match updated_vote {
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
                                return HttpResponse::Ok().body("Vote successfully updated");
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
                                error!("Error updating vote: {}", err);
                                return HttpResponse::InternalServerError()
                                    .body("Error updating vote.");
                            }
                        }
                    }
                }
                Ok(None) => {
                    //create vote
                    let mut tx = db_pool.begin().await.unwrap();
                    let formatted_form = CommentVote {
                        comment_id: vote_form.comment_id,
                        user_id: user.user_id,
                        up: vote_form.up,
                    };
                    let created_vote = CommentVote::create(&formatted_form, &mut tx).await;
                    match created_vote {
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
                            return HttpResponse::Ok().body("Vote successful.");
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
                            error!("Error creating vote: {}", err);
                            return HttpResponse::InternalServerError().body("Voting error.");
                        }
                    }
                }
                Err(err) => {
                    error!("Error fetching comment vote: {}", err);
                    return HttpResponse::InternalServerError()
                        .body("Error fetching comment votes.");
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
