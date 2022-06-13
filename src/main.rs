#[macro_use]
extern crate log;

use actix_session::CookieSession;
use actix_web::{middleware, web, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use sqlx::postgres::PgPool;

mod aggregates;
mod block;
mod comment;
mod comment_vote;
mod guild;
mod guild_membership;
mod notification;
mod password_reset;
mod post;
mod post_vote;
mod report;
mod routes;
mod search;
mod site;
mod user;
mod user_registration;
mod user_session;
mod utils;
mod view;
//mod bookmark;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let database_url =
        dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set in the .env file");
    info!("using postgres database at: {}", &database_url);
    let db_pool = PgPool::connect(&database_url).await?;

    let server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            //TODO: MAKE THIS COOKIE ENCRYPTED
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .configure(routes::registration::init)
            .service(web::scope("/user").configure(routes::user::init))
            .service(web::scope("/guild").configure(routes::guild::init))
            .service(web::scope("/post").configure(routes::post::init))
            .service(web::scope("/comment").configure(routes::comment::init))
            .service(web::scope("/vote").configure(routes::vote::init))
            .service(web::scope("/resetpassword").configure(routes::reset_password::init))
            .service(web::scope("/admin").configure(routes::site::init))
            .service(web::scope("/report").configure(routes::report::init))
            .service(web::scope("/view").configure(routes::view::init))
    })
    .bind("127.0.0.1:4567")?;

    info!("Starting server at port 4567");
    server.run().await?;

    Ok(())
}
