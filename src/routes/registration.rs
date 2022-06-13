use crate::user_registration::api_handlers;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(api_handlers::register::handler))
        .service(api_handlers::confirm_registration::handler);
}
