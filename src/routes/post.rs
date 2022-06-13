use crate::post::api_handlers;
use actix_web::web;

//all these routes are preceded by the namespaced /guild

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/create",
        web::post().to(api_handlers::create_post::handler),
    )
    .route("/edit", web::post().to(api_handlers::edit_post::handler))
    .route(
        "/delete",
        web::post().to(api_handlers::delete_post::handler),
    )
    .route("/lock", web::post().to(api_handlers::lock_post::handler))
    .route(
        "/unlock",
        web::post().to(api_handlers::unlock_post::handler),
    );
}
