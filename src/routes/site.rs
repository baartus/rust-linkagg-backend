use crate::site::api_handlers;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(api_handlers::site_ban_user::handler)
        .service(api_handlers::site_unban_user::handler)
        .service(api_handlers::site_delete_user::handler)
        .service(api_handlers::site_verify_user::handler)
        .service(api_handlers::site_unverify_user::handler)
        .service(api_handlers::site_make_user_admin::handler);
}
