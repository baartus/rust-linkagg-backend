use crate::view::api_handlers;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(api_handlers::get_all_posts::handler)
        .service(api_handlers::get_guild_details::handler)
        .service(api_handlers::get_posts_by_guild::handler)
        .service(api_handlers::get_user_detailed::handler)
        .service(api_handlers::get_user_comments::handler)
        .service(api_handlers::get_user_posts::handler)
        .service(api_handlers::get_post_comments::handler)
        .service(api_handlers::get_user_personal_info::handler)
        .service(api_handlers::get_short_guild_details::handler);
}
