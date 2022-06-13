use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::user::User;
use crate::user_registration::*;

pub async fn handler(
    registration_form: web::Json<UserRegistrationForm>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    //make sure passwords match
    if registration_form.password != registration_form.confirm_password {
        return HttpResponse::InternalServerError().body("passwords do not match");
    }

    //validate username, must be all alphanumeric, 15 chars max
    let mut valid_username: bool;
    valid_username = registration_form
        .username
        .chars()
        .all(char::is_alphanumeric);
    if registration_form.username.len() > 15 {
        valid_username = false;
    }

    if !valid_username {
        return HttpResponse::NotAcceptable()
                .body("Usernames can only contain alphanumeric characters, and must be no longer than 15 characters");
    }

    //TODO: Validate email (proabbly use regex...)

    let cloned_form = registration_form.clone();

    let formatted_form = UserRegistrationForm {
        username: cloned_form.username.to_lowercase(),
        email: cloned_form.email.to_lowercase(),
        password: cloned_form.password,
        confirm_password: cloned_form.confirm_password,
    };

    //check for existing usernames, emailscontains(char::is_alphanumeric);

    let existing_username =
        User::find_by_username(&formatted_form.username, db_pool.get_ref()).await;
    match existing_username {
        Ok(Some(user)) => {
            return HttpResponse::InternalServerError()
                .body("Error registering user: User with that username already exists");
        }
        Err(err) => {
            error!("Error registering user: {}", err);
            return HttpResponse::InternalServerError().body("Error registering user");
        }
        _ => (),
    }

    let existing_username_registration =
        UserRegistration::find_by_username(&formatted_form.username, db_pool.get_ref()).await;
    match existing_username_registration {
        Ok(Some(user_registration)) => {
            return HttpResponse::InternalServerError()
                .body("Error registering user: That username has already been registered");
        }
        Err(err) => {
            error!("Error registering user: {}", err);
            return HttpResponse::InternalServerError().body("Error registering user");
        }
        _ => (),
    }

    let existing_email = User::find_by_email(&formatted_form.email, db_pool.get_ref()).await;
    match existing_email {
        Ok(Some(user)) => {
            return HttpResponse::InternalServerError()
                .body("Error registering user: User with that email already exists");
        }
        Err(err) => {
            error!("Error registering user: {}", err);
            return HttpResponse::InternalServerError().body("Error registering user");
        }
        _ => (),
    }

    let existing_email_registration =
        UserRegistration::find_by_email(&formatted_form.email, db_pool.get_ref()).await;
    match existing_email_registration {
        Ok(Some(user_registration)) => {
            return HttpResponse::InternalServerError()
                .body("Error registering user: That email has already been registered");
        }
        Err(err) => {
            error!("Error registering user: {}", err);
            return HttpResponse::InternalServerError().body("Error registering user");
        }
        _ => (),
    }

    //TODO: validate email address

    //email and username are open, begin tx

    let mut tx = db_pool.begin().await.unwrap();

    //username is free, register user
    let registration = UserRegistration::create(&formatted_form, &mut tx).await;
    match registration {
        Ok(registration_details) => {
            //TODO: IMPORT MAILER CRATE, EMAIL HASH TO THE EMAIL
            let succesful_commit = tx.commit().await;
            match succesful_commit {
                Ok(()) => (),
                Err(err) => {
                    error!("Error committing transaction: {}", err);
                    return HttpResponse::InternalServerError().body("Unknown Error.");
                }
            }
            HttpResponse::Ok().json(registration_details)
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
            HttpResponse::InternalServerError().body("Error registering user")
        }
    }
}
