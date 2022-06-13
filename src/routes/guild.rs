use crate::guild::api_handlers;
use actix_web::web;

//all these routes are preceded by the namespaced /guild

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/create",
        web::post().to(api_handlers::create_guild::handler),
    )
    .service(api_handlers::join_guild::handler)
    .service(api_handlers::leave_guild::handler)
    .service(api_handlers::remove_guild::handler)
    .service(api_handlers::remove_moderator::handler)
    .service(api_handlers::appoint_moderator::handler)
    .route(
        "mod/removepost",
        web::post().to(api_handlers::mod_remove_post::handler),
    )
    .route(
        "/mod/removecomment",
        web::post().to(api_handlers::mod_remove_comment::handler),
    )
    .service(api_handlers::update_guild_name::handler)
    .service(api_handlers::update_guild_description::handler)
    .service(api_handlers::update_guild_banner::handler)
    .service(api_handlers::update_guild_avatar::handler);
}
