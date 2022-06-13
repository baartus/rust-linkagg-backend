use crate::comment_vote::api_handlers as comment_vote_handlers;
use crate::post_vote::api_handlers as post_vote_handlers;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/post", web::post().to(post_vote_handlers::vote::handler))
        .route(
            "/comment",
            web::post().to(comment_vote_handlers::vote::handler),
        );
}
