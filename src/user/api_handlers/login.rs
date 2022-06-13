use crate::user::*;
use crate::user_session::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn handler(
    login_form: web::Json<UserLoginForm>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    //check if there is a session in cookie
    let valid = session_validation::validate_session(&session, db_pool.get_ref()).await;
    match valid {
        Ok(Some(user)) => {
            return HttpResponse::InternalServerError()
                .body("Login error. User is already logged in");
        }
        Err(err) => {
            error!("Error validating session: {}", err);
            return HttpResponse::InternalServerError().body("Login error.");
        }
        Ok(None) => (),
    }

    //format login form
    let cloned_form = login_form.clone();
    let formatted_form = UserLoginForm {
        username: cloned_form.username.to_lowercase(),
        password: cloned_form.password,
    };

    //check to make sure user with username exists
    let user_exists =
        User::find_by_username_sensitive(&formatted_form.username, db_pool.get_ref()).await;
    match user_exists {
        Ok(None) => {
            return HttpResponse::InternalServerError()
                .body("A user with that username does not exist");
        }
        Err(err) => {
            error!("Error finding user: {}", err);
            return HttpResponse::InternalServerError().body("Login error.");
        }
        Ok(Some(user)) => {
            //user exists
            if user.is_banned {
                return HttpResponse::Forbidden().body("You are banned.");
            }
            //check that password matches
            let password_check = User::verify_password(&user, &formatted_form.password).await;
            match password_check {
                Ok(true) => {
                    //password matches
                    //create session
                    let mut tx = db_pool.begin().await.unwrap();
                    let created_session = UserSession::create(user.user_id, &mut tx).await;
                    match created_session {
                        Ok(new_session) => {
                            //store session id in cookie
                            let create_session = session.set("session_id", &new_session.session_id);
                            match create_session {
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
                                    return HttpResponse::Ok().json(UserSessionView {
                                        session_id: new_session.session_id,
                                        user_id: new_session.user_id,
                                    });
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
                                    error!("Error creating session cookie: {}", err);
                                    return HttpResponse::InternalServerError().body("Login error");
                                }
                            }
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
                            error!("Error creating session in db: {}", err);
                            return HttpResponse::InternalServerError().body("Login error");
                        }
                    }
                }
                Ok(false) => {
                    //password doesnt match
                    return HttpResponse::InternalServerError().body("Incorrect password");
                }
                Err(err) => {
                    error!("Error verifying password: {}", err);
                    return HttpResponse::InternalServerError().body("Login error.");
                }
            }
        }
    }
}
