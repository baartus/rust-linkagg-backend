use crate::user::api_handlers;
use actix_web::web;

//all these routes are preceded by the namespaced /user

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/login", web::post().to(api_handlers::login::handler))
        .route("/logout", web::post().to(api_handlers::logout::handler))
        .service(api_handlers::block_user::handler)
        .service(api_handlers::unblock_user::handler);
}
