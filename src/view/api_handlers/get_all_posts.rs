use crate::block::Block;
use crate::post_vote::PostVote;
use crate::utils::session_validation;
use crate::view::DetailedPostView;
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/posts/{page_number}")]
pub async fn handler(
    db_pool: web::Data<PgPool>,
    page_number: web::Path<i64>,
    session: Session,
) -> impl Responder {
    let is_user = session_validation::policy_user(&session, db_pool.get_ref()).await;
    match is_user {
        Ok((None, Some(user))) => {
            //user
            let get_posts =
                DetailedPostView::get_all_posts(db_pool.get_ref(), &20, &page_number).await;
            match get_posts {
                Ok(posts) => {
                    //get user block list
                    let block_list =
                        Block::find_all_by_user(&user.user_id, db_pool.get_ref()).await;
                    match block_list {
                        Ok(blocks) => {
                            let mut the_posts: Vec<DetailedPostView> = Vec::new();
                            for mut post in posts {
                                for block in &blocks {
                                    match &post.username {
                                        Some(username) => {
                                            if &block.blocked_user_username == username {
                                                post.is_blocked = true;
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                                match &post.post_id {
                                    Some(post_id) => {
                                        let is_voted = PostVote::find_by_post_and_user_id(
                                            post_id,
                                            &user.user_id,
                                            db_pool.get_ref(),
                                        )
                                        .await;
                                        match is_voted {
                                            Ok(Some(vote)) => {
                                                if vote.up {
                                                    post.is_upvoted = true;
                                                } else {
                                                    post.is_downvoted = true;
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
                                the_posts.push(post.clone());
                            }
                            return HttpResponse::Ok().json(the_posts);
                        }
                        Err(err) => {
                            error!("Error fetching posts: {}", err);
                            HttpResponse::InternalServerError().body("Error fetching block list.")
                        }
                    }
                }
                Err(err) => {
                    error!("Error fetching posts: {}", err);
                    HttpResponse::InternalServerError().body("Error fetching posts.")
                }
            }
        }
        Ok((Some(response), None)) => {
            //not user
            let get_posts =
                DetailedPostView::get_all_posts(db_pool.get_ref(), &20, &page_number).await;
            match get_posts {
                Ok(posts) => HttpResponse::Ok().json(posts),
                Err(err) => {
                    error!("Error fetching posts: {}", err);
                    HttpResponse::InternalServerError().body("Error fetching posts.")
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
