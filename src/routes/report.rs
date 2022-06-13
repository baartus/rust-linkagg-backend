use crate::report::api_handlers;
use actix_web::web;

//all these routes are preceded by the namespaced /guild

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/create",
        web::post().to(api_handlers::create_report::handler),
    );
}
