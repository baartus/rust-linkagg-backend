use crate::comment::*;
use crate::post::*;
use crate::report::*;
use crate::utils::session_validation;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn handler(
    report_form: web::Json<ReportForm>,
    db_pool: web::Data<PgPool>,
    session: Session,
) -> impl Responder {
    if report_form.reason == "" {
        return HttpResponse::BadRequest()
            .body("You must enter a reason you are reporting the post");
    }
    let valid_session = session_validation::policy_user(&session, db_pool.get_ref()).await;
    match valid_session {
        Ok((None, Some(user))) => {
            if report_form.post_id == 0 && report_form.comment_id == 0 {
                return HttpResponse::BadRequest().body("You must select a post to report.");
            } else if report_form.post_id != 0 && report_form.comment_id != 0 {
                return HttpResponse::BadRequest()
                    .body("You can only report one post/comment at a time.");
            }
            if report_form.post_id == 0 {
                //make sure comment exists
                let comment_exists =
                    Comment::find_by_comment_id(&report_form.comment_id, db_pool.get_ref()).await;
                match comment_exists {
                    Ok(Some(comment)) => {
                        //create report
                        let mut tx = db_pool.begin().await.unwrap();
                        let created_report = Report::create(&report_form, &mut tx).await;
                        match created_report {
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
                                return HttpResponse::Ok().body("Report submitted.");
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
                                error!("Error reporting comment: {}", err);
                                return HttpResponse::InternalServerError()
                                    .body("Error reporting comment.");
                            }
                        }
                    }
                    Ok(None) => {
                        return HttpResponse::BadRequest()
                            .body("The post you are trying to report does not exist");
                    }
                    Err(err) => {
                        error!("Error reporting comment: {}", err);
                        return HttpResponse::InternalServerError()
                            .body("Error reporting comment.");
                    }
                }
            } else {
                //make sure post exists
                let post_exists =
                    Post::find_by_post_id(&report_form.post_id, db_pool.get_ref()).await;
                match post_exists {
                    Ok(Some(post)) => {
                        //create report
                        let mut tx = db_pool.begin().await.unwrap();
                        let created_report = Report::create(&report_form, &mut tx).await;
                        match created_report {
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
                                return HttpResponse::Ok().body("Report submitted.");
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
                                error!("Error reporting post: {}", err);
                                return HttpResponse::InternalServerError()
                                    .body("Error reporting post.");
                            }
                        }
                    }
                    Ok(None) => {
                        return HttpResponse::BadRequest()
                            .body("The post you are trying to report does not exist");
                    }
                    Err(err) => {
                        error!("Error reporting post: {}", err);
                        return HttpResponse::InternalServerError().body("Error reporting post.");
                    }
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
