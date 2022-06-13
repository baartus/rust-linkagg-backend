use crate::view::DetailedUserView;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/user/{username}")]
pub async fn handler(db_pool: web::Data<PgPool>, username: web::Path<String>) -> impl Responder {
    let formatted_username = username.to_string().to_lowercase();
    let user_exists =
        DetailedUserView::find_by_username(&formatted_username, db_pool.get_ref()).await;
    match user_exists {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::BadRequest().body("User does not exist."),
        Err(err) => {
            error!("Error fetching user: {}", err);
            HttpResponse::InternalServerError().body("Error fetching user.")
        }
    }
}
