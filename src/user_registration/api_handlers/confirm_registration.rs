use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::user::{User, UserCreateForm};
use crate::user_registration::*;

#[get("/confirmregistration/{registration_hash}")]
pub async fn handler(
    registration_hash: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let mut tx = db_pool.begin().await.unwrap();
    let find_registration =
        UserRegistration::find_by_hash(registration_hash.to_string(), db_pool.get_ref()).await;
    match find_registration {
        Ok(Some(registration)) => {
            let registration_data = registration.clone();
            //move registered user into user
            let user_create_form = UserCreateForm {
                email: registration_data.email,
                username: registration_data.username,
                password_hash: registration_data.password_hash,
            };
            let create_user = User::create(&user_create_form, &mut tx).await;
            match create_user {
                Ok(()) => {
                    //delete user registration
                    let delete_registration = UserRegistration::delete(registration, &mut tx).await;
                    match delete_registration {
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
                            HttpResponse::Ok().body("User has been validated")
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
                            error!("Error registering user: {}", err);
                            HttpResponse::InternalServerError().body("Error validating user")
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
                    error!("Error registering user: {}", err);
                    HttpResponse::InternalServerError().body("Error validating user")
                }
            }
        }
        Ok(None) => {
            let succesful_rollback = tx.rollback().await;
            match succesful_rollback {
                Ok(()) => (),
                Err(err) => {
                    error!("Error rolling back transaction: {}", err);
                    return HttpResponse::InternalServerError().body("Unknown Error.");
                }
            }
            HttpResponse::InternalServerError()
                .body("Error validating user. Double check link in email")
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
            error!("Error registering user: {}", err);
            HttpResponse::InternalServerError().body("Error validating user")
        }
    }
}
