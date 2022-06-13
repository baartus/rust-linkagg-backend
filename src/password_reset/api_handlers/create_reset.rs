use crate::password_reset::*;
use crate::user::*;
use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

#[post("/{username}")]
pub async fn handler(username: web::Path<String>, db_pool: web::Data<PgPool>) -> impl Responder {
    let user_exists = User::find_by_username_sensitive(&username, db_pool.get_ref()).await;
    match user_exists {
        Ok(Some(user)) => {
            //create password reset request & email hash to user
            if user.is_banned {
                return HttpResponse::BadRequest()
                    .body("The user you are requesting a password reset for is banned.");
            }
            let mut tx = db_pool.begin().await.unwrap();
            let new_reset = PasswordReset::create_reset(&user.user_id, &mut tx).await;
            match new_reset {
                Ok(reset_hash) => {
                    //TODO: mail reset hash to user

                    let succesful_commit = tx.commit().await;
                    match succesful_commit {
                        Ok(()) => (),
                        Err(err) => {
                            error!("Error committing transaction: {}", err);
                            return HttpResponse::InternalServerError().body("Unknown Error.");
                        }
                    }
                    HttpResponse::Ok()
                        .body("A link to reset your password has been sent to your email.")
                }
                Err(err) => {
                    error!("Error creating a password reset request: {}", err);
                    return HttpResponse::InternalServerError()
                        .body("Error starting a password reset.");
                }
            }
        }
        Ok(None) => {
            return HttpResponse::BadRequest()
                .body("The user you are requesting a password reset for does not exist.");
        }
        Err(err) => {
            error!("Error fetching user: {}", err);
            return HttpResponse::InternalServerError().body("Error fetching user.");
        }
    }
}
