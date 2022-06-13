use crate::password_reset::api_handlers;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(api_handlers::create_reset::handler)
        .service(api_handlers::verify_reset::handler)
        .service(api_handlers::change_password::handler);
}
