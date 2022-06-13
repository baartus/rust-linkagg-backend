use crate::guild_membership::GuildMembership;
use crate::utils::session_validation;
use crate::view::ShortGuildView;
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/guilds/{page_number}")]
pub async fn handler(
    db_pool: web::Data<PgPool>,
    page_number: web::Path<i64>,
    session: Session,
) -> impl Responder {
    let is_user = session_validation::policy_user(&session, db_pool.get_ref()).await;
    match is_user {
        Ok((None, Some(user))) => {
            //is user
            let get_guilds = ShortGuildView::find_all(&20, &page_number, db_pool.get_ref()).await;
            match get_guilds {
                Ok(guilds) => {
                    //clone a mutable guild vector
                    //get user guild memberships
                    let get_memberships = GuildMembership::find_all_by_user_id(
                        &user.user_id,
                        db_pool.get_ref(),
                        &0,
                        &1,
                    )
                    .await;
                    match get_memberships {
                        Ok(memberships) => {
                            //for each guild in guild vector, check in user guild memberships if they are a member. if so, set is_member to true.
                            let mut the_guilds: Vec<ShortGuildView> = Vec::new();
                            for mut guild in guilds {
                                for membership in &memberships {
                                    match &guild.guild_tag {
                                        Some(tag) => {
                                            if &membership.guild_tag == tag {
                                                guild.is_member = true;
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                                the_guilds.push(guild.clone());
                            }
                            return HttpResponse::Ok().json(the_guilds);
                        }
                        Err(err) => {
                            error!("Error fetching guild memberships: {}", err);
                            return HttpResponse::InternalServerError()
                                .body("Error fetching guild memberships.");
                        }
                    }
                }
                Err(err) => {
                    error!("Error fetching guild: {}", err);
                    return HttpResponse::InternalServerError().body("Error fetching guilds.");
                }
            }
        }
        Ok((Some(response), None)) => {
            //is not user
            let get_guilds = ShortGuildView::find_all(&20, &page_number, db_pool.get_ref()).await;
            match get_guilds {
                Ok(guilds) => {
                    return HttpResponse::Ok().json(guilds);
                }
                Err(err) => {
                    error!("Error fetching guild: {}", err);
                    return HttpResponse::InternalServerError().body("Error fetching guilds.");
                }
            }
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
