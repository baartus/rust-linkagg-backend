use crate::password_reset::*;
use crate::user::*;
use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

#[post("/update/{reset_hash}")]
pub async fn handler(
    reset_hash: web::Path<String>,
    reset_form: web::Json<PasswordResetForm>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    if reset_form.new_password != reset_form.confirm_new_password {
        return HttpResponse::BadRequest().body("Passwords do not match.");
    }

    //TODO: password strength validation

    let reset_exists = PasswordReset::find_reset_by_hash(&reset_hash, db_pool.get_ref()).await;
    match reset_exists {
        Ok(Some(reset)) => {
            if !reset.verified_email {
                //todo: maybe this should be more vague...
                return HttpResponse::BadRequest().body(
                    "You have not verified your email. Click the link in the email to do so.",
                );
            }
            let mut tx = db_pool.begin().await.unwrap();
            let changed_password =
                User::update_password(&reset_form.new_password, &reset.user_id, &mut tx).await;
            match changed_password {
                Ok(()) => {
                    //delete password reset
                    let deleted_reset = PasswordReset::delete(&reset.user_id, &mut tx).await;
                    match deleted_reset {
                        Ok(()) => {
                            let succesful_commit = tx.commit().await;
                            match succesful_commit {
                                Ok(()) => (),
                                Err(err) => {
                                    error!("Error committing transaction: {}", err);
                                    return HttpResponse::InternalServerError()
                                        .body("Unknown Error.");
                                }
                            }
                            return HttpResponse::Ok().body("Password changed successfully.");
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
                            error!("Error deleting password reset: {}", err);
                            return HttpResponse::InternalServerError()
                                .body("Unknown Error changing password.");
                        }
                    }
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
                    error!("Error changing password: {}", err);
                    return HttpResponse::InternalServerError()
                        .body("Unknown Error changing password.");
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
