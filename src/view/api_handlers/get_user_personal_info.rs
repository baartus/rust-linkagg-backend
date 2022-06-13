use crate::utils::session_validation;
use crate::view::UserPersonalView;
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/user/{username}/personal")]
pub async fn handler(
    db_pool: web::Data<PgPool>,
    username: web::Path<String>,
    session: Session,
) -> impl Responder {
    let formatted_username = username.to_string().to_lowercase();
    let valid_session =
        session_validation::policy_self(&formatted_username, &session, db_pool.get_ref()).await;
    match valid_session {
        Ok((None, Some(user))) => {
            let user_exists =
                UserPersonalView::find_by_username(&formatted_username, db_pool.get_ref()).await;
            match user_exists {
                Ok(Some(user)) => HttpResponse::Ok().json(user),
                Ok(None) => HttpResponse::BadRequest().body("User does not exist."),
                Err(err) => {
                    error!("Error fetching user: {}", err);
                    HttpResponse::InternalServerError().body("Error fetching user.")
                }
            }
        }
        Ok((Some(response), None)) => {
            return response;
        }
        Err(err) => {
            error!("Error verifying user session: {}", err);
            return HttpResponse::InternalServerError().body("Error verifying user session.");
        }
        _ => {
            return HttpResponse::InternalServerError().body("Unknown Error.");
        }
    }
}
