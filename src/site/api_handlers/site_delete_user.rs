use crate::user::User;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

#[post("/delete/{username}")]
pub async fn handler(
    username: web::Path<String>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    let formatted_username = username.to_string().to_lowercase();
    let valid_session = session_validation::policy_admin(&session, db_pool.get_ref()).await;
    match valid_session {
        Ok((None, Some(user))) => {
            //make sure user exists
            let user_exists =
                User::find_by_username_sensitive(&formatted_username, db_pool.get_ref()).await;
            match user_exists {
                Ok(Some(user_to_delete)) => {
                    //delete user
                    if user_to_delete.is_admin {
                        return HttpResponse::Forbidden().body("You cannot delete a site admin.");
                    }
                    let mut tx = db_pool.begin().await.unwrap();
                    let delete_success = User::delete(&user_to_delete.user_id, &mut tx).await;
                    match delete_success {
                        Ok(()) => {
                            tx.commit().await.unwrap();
                            return HttpResponse::Ok().body("User has been deleted.");
                        }
                        Err(err) => {
                            let succesful_rollback = tx.rollback().await;
                            match succesful_rollback {
                                Ok(()) => (),
                                Err(err) => {
                                    error!("Error rolling back transaction: {}", err);
                                    return HttpResponse::InternalServerError()
                                        .body("Unknown Error.");
                                }
                            }
                            error!("Error deleting user: {}", err);
                            return HttpResponse::InternalServerError()
                                .body("Error deleting user.");
                        }
                    }
                }
                Ok(None) => {
                    return HttpResponse::BadRequest()
                        .body("User you are trying to delete does not exist.");
                }
                Err(err) => {
                    error!("Error fetching user: {}", err);
                    return HttpResponse::InternalServerError().body("Error fetching user.");
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
