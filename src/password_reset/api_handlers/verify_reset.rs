use crate::password_reset::*;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/verify/{reset_hash}")]
pub async fn handler(reset_hash: web::Path<String>, db_pool: web::Data<PgPool>) -> impl Responder {
    let reset_exists = PasswordReset::find_reset_by_hash(&reset_hash, db_pool.get_ref()).await;
    match reset_exists {
        Ok(Some(reset)) => {
            if reset.verified_email {
                return HttpResponse::BadRequest()
                    .body("You have already verified your email. Proceed to reset password");
            }
            let mut tx = db_pool.begin().await.unwrap();
            let verified_reset = PasswordReset::verify_reset(&reset.user_id, &mut tx).await;
            match verified_reset {
                Ok(()) => {
                    let succesful_commit = tx.commit().await;
                    match succesful_commit {
                        Ok(()) => (),
                        Err(err) => {
                            error!("Error committing transaction: {}", err);
                            return HttpResponse::InternalServerError().body("Unknown Error.");
                        }
                    }
                    //probably redirect here
                    return HttpResponse::Ok().body("identity verified. proceed to reset password");
                }
                Err(err) => {
                    let succesful_rollback = tx.rollback().await;
                    match succesful_rollback {
                        Ok(()) => (),
                        Err(err) => {
                            error!("Error rolling back transaction: {}", err);
                            return HttpResponse::InternalServerError().body("Unknown Error.");
                        }
                    }
                    error!("Verification error: {}", err);
                    return HttpResponse::InternalServerError().body("Unknown Verification Error.");
                }
            }
        }
        Ok(None) => {
            return HttpResponse::BadRequest().body("The reset URL you have requested is invalid.");
        }
        Err(err) => {
            error!("Error fetching password reset request: {}", err);
            return HttpResponse::InternalServerError()
                .body("Error fetching password reset request.");
        }
    }
}
