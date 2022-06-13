use crate::user_session::UserSession;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn handler(db_pool: web::Data<PgPool>, session: Session) -> impl Responder {
    //check to make sure user has session key in cookie
    if let Ok(Some(session_id)) = session.get::<String>("session_id") {
        //check that session key matches active session in db
        let valid_session = UserSession::find_by_session_id(session_id, db_pool.get_ref()).await;

        match valid_session {
            Ok(Some(current_session)) => {
                //session is valid, remove from db and cookie
                let mut tx = db_pool.begin().await.unwrap();
                let deleted = UserSession::delete(current_session, &mut tx).await;
                match deleted {
                    Ok(()) => {
                        let succesful_commit = tx.commit().await;
                        match succesful_commit {
                            Ok(()) => (),
                            Err(err) => {
                                error!("Error committing transaction: {}", err);
                                return HttpResponse::InternalServerError().body("Unknown Error.");
                            }
                        }

                        //remove from cookie
                        session.remove("session_id");

                        return HttpResponse::Ok().body("Logged out successfully");
                    }
                    Err(err) => {
                        error!("Error deleting session on log out: {}", err);
                        return HttpResponse::InternalServerError().body("Error logging out.");
                    }
                }
            }
            Ok(None) => {
                //session is invalid, just remove from cookie
                session.remove("session_id");
                return HttpResponse::Ok().body("Logged out of expired session");
            }
            Err(err) => {
                error!("Error checking session on log out: {}", err);
                return HttpResponse::InternalServerError().body("Error logging out.");
            }
        }
        //delete session
        //remove session from cookie
    }
    return HttpResponse::InternalServerError().body("Error. You are already logged out.");
}
