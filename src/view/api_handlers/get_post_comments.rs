use crate::block::Block;
use crate::comment_vote::CommentVote;
use crate::utils::session_validation;
use crate::view::DetailedCommentView;
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct GetPostComments {
    post_id: i32,
    page_number: i64,
}

#[get("/post/{post_id}/comments/{page_number}")]
pub async fn handler(
    db_pool: web::Data<PgPool>,
    request_form: web::Path<GetPostComments>,
    session: Session,
) -> impl Responder {
    let is_user = session_validation::policy_user(&session, db_pool.get_ref()).await;
    match is_user {
        Ok((None, Some(user))) => {
            //user
            let get_comments = DetailedCommentView::get_comments_by_post_id(
                &request_form.post_id,
                db_pool.get_ref(),
                &20,
                &request_form.page_number,
            )
            .await;
            match get_comments {
                Ok(comments) => {
                    //get user block list
                    let block_list =
                        Block::find_all_by_user(&user.user_id, db_pool.get_ref()).await;
                    match block_list {
                        Ok(blocks) => {
                            let mut the_comments: Vec<DetailedCommentView> = Vec::new();
                            for mut comment in comments {
                                for block in &blocks {
                                    match &comment.username {
                                        Some(username) => {
                                            if &block.blocked_user_username == username {
                                                comment.is_blocked = true;
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                                match &comment.post_id {
                                    Some(comment_id) => {
                                        let is_voted = CommentVote::find_by_comment_and_user_id(
                                            comment_id,
                                            &user.user_id,
                                            db_pool.get_ref(),
                                        )
                                        .await;
                                        match is_voted {
                                            Ok(Some(vote)) => {
                                                if vote.up {
                                                    comment.is_upvoted = true;
                                                } else {
                                                    comment.is_downvoted = true;
                                                }
                                            }
                                            Err(err) => {
                                                error!("Error fetching votes: {}", err);
                                                HttpResponse::InternalServerError()
                                                    .body("Error fetching votes.");
                                            }
                                            _ => (),
                                        }
                                    }
                                    _ => (),
                                }
                                the_comments.push(comment.clone());
                            }
                            return HttpResponse::Ok().json(the_comments);
                        }
                        Err(err) => {
                            error!("Error fetching posts: {}", err);
                            HttpResponse::InternalServerError().body("Error fetching block list.")
                        }
                    }
                }
                Err(err) => {
                    error!("Error fetching comments: {}", err);
                    HttpResponse::InternalServerError().body("Error fetching comments.")
                }
            }
        }
        Ok((Some(response), None)) => {
            //not user
            let get_comments = DetailedCommentView::get_comments_by_post_id(
                &request_form.post_id,
                db_pool.get_ref(),
                &20,
                &request_form.page_number,
            )
            .await;
            match get_comments {
                Ok(comments) => HttpResponse::Ok().json(comments),
                Err(err) => {
                    error!("Error fetching comments: {}", err);
                    HttpResponse::InternalServerError().body("Error fetching comments.")
                }
            }
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
