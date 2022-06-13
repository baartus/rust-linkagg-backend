use crate::guild_membership::GuildMembership;
use crate::user::User;
use crate::user_session::UserSession;
use actix_session::Session;
use actix_web::HttpResponse;
use anyhow::Result;
use sqlx::PgPool;

pub async fn validate_session(session: &Session, pool: &PgPool) -> Result<Option<User>> {
    if let Ok(Some(session_id)) = session.get::<String>("session_id") {
        let active_session = UserSession::find_by_session_id(session_id, pool).await;
        match active_session {
            //make sure session is valid
            Ok(Some(my_session)) => {
                //session is valid
                //get user
                let find_user = User::find_by_id(&my_session.user_id, pool).await;
                match find_user {
                    Ok(Some(user)) => {
                        //figure out logic here for banned users...
                        return Ok(Some(user));
                    }
                    Ok(None) => {
                        return Ok(None);
                    }
                    Err(err) => {
                        error!("Error finding user: {}", err);
                        return Ok(None);
                    }
                }
            }
            Ok(None) => {
                //remove session from cookie
                session.remove("session_id");
                return Ok(None);
            }
            Err(err) => {
                error!("Error finding user: {}", err);
                return Ok(None);
            }
        }
    }

    Ok(None)
}

pub async fn policy_user(
    session: &Session,
    pool: &PgPool,
) -> Result<(Option<HttpResponse>, Option<User>)> {
    let valid_session = validate_session(session, pool).await;
    match valid_session {
        Ok(Some(user)) => {
            if user.is_banned {
                Ok((
                    Some(HttpResponse::Forbidden().body("You are banned.")),
                    None,
                ))
            } else {
                Ok((None, Some(user)))
            }
        }
        Ok(None) => Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None)),
        Err(err) => {
            error!(
                "Error checking guild membership data on admin policy: {}",
                err
            );
            Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None))
        }
    }
}

pub async fn policy_self(
    username: &String,
    session: &Session,
    pool: &PgPool,
) -> Result<(Option<HttpResponse>, Option<User>)> {
    let valid_session = validate_session(session, pool).await;
    match valid_session {
        Ok(Some(user)) => {
            if user.is_banned {
                Ok((
                    Some(HttpResponse::Forbidden().body("You are banned.")),
                    None,
                ))
            } else {
                if &user.username == username {
                    Ok((None, Some(user)))
                } else {
                    Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None))
                }
            }
        }
        Ok(None) => Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None)),
        Err(err) => {
            error!(
                "Error checking guild membership data on admin policy: {}",
                err
            );
            Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None))
        }
    }
}

pub async fn policy_admin(
    session: &Session,
    pool: &PgPool,
) -> Result<(Option<HttpResponse>, Option<User>)> {
    let valid_session = validate_session(session, pool).await;
    match valid_session {
        Ok(Some(user)) => match user.is_admin {
            true => Ok((None, Some(user))),
            _ => Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None)),
        },
        Ok(none) => Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None)),
        Err(err) => {
            error!(
                "Error checking guild membership data on admin policy: {}",
                err
            );
            Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None))
        }
    }
}

pub async fn policy_guild_admin(
    session: &Session,
    guild_tag: &String,
    pool: &PgPool,
) -> Result<(Option<HttpResponse>, Option<User>)> {
    let valid_session = validate_session(session, pool).await;
    match valid_session {
        Ok(Some(user)) => {
            if user.is_banned {
                return Ok((
                    Some(HttpResponse::Forbidden().body("You are banned.")),
                    None,
                ));
            }

            //also make avaiable to site admins
            if user.is_admin {
                return Ok((None, Some(user)));
            }
            let is_member =
                GuildMembership::find_by_user_and_guild_tag(&user.user_id, guild_tag, pool).await;
            match is_member {
                Ok(Some(membership)) => match membership.is_admin {
                    true => Ok((None, Some(user))),
                    _ => Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None)),
                },
                Ok(None) => Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None)),
                Err(err) => {
                    error!(
                        "Error checking guild membership data on admin policy: {}",
                        err
                    );
                    Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None))
                }
            }
        }
        Ok(None) => Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None)),
        Err(err) => {
            error!(
                "Error checking guild membership data on admin policy: {}",
                err
            );
            Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None))
        }
    }
}

pub async fn policy_guild_member(
    session: &Session,
    guild_tag: &String,
    pool: &PgPool,
) -> Result<(Option<HttpResponse>, Option<User>)> {
    let valid_session = validate_session(session, pool).await;
    match valid_session {
        Ok(Some(user)) => {
            if user.is_banned {
                return Ok((
                    Some(HttpResponse::Forbidden().body("You are banned.")),
                    None,
                ));
            }
            //also make avaiable to site admins
            if user.is_admin {
                return Ok((None, Some(user)));
            }
            let is_member =
                GuildMembership::find_by_user_and_guild_tag(&user.user_id, guild_tag, pool).await;
            match is_member {
                Ok(Some(membership)) => {
                    if membership.is_banned {
                        Ok((
                            Some(HttpResponse::Forbidden().body("You are banned from this guild.")),
                            None,
                        ))
                    } else {
                        Ok((None, Some(user)))
                    }
                }
                Ok(None) => Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None)),
                Err(err) => {
                    error!("Error checking guild membership data on policy: {}", err);
                    Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None))
                }
            }
        }
        Ok(None) => Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None)),
        Err(err) => {
            error!(
                "Error checking guild membership data on admin policy: {}",
                err
            );
            Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None))
        }
    }
}

pub async fn policy_guild_moderator_or_admin(
    session: &Session,
    guild_tag: &String,
    pool: &PgPool,
) -> Result<(Option<HttpResponse>, Option<User>)> {
    let valid_session = validate_session(session, pool).await;
    match valid_session {
        Ok(Some(user)) => {
            if user.is_banned {
                return Ok((
                    Some(HttpResponse::Forbidden().body("You are banned.")),
                    None,
                ));
            }
            //also make avaiable to site admins
            if user.is_admin {
                return Ok((None, Some(user)));
            }
            let is_member =
                GuildMembership::find_by_user_and_guild_tag(&user.user_id, guild_tag, pool).await;
            match is_member {
                Ok(Some(membership)) => {
                    if membership.is_admin || membership.is_moderator {
                        Ok((None, Some(user)))
                    } else {
                        Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None))
                    }
                }
                Ok(None) => Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None)),
                Err(err) => {
                    error!(
                        "Error checking guild membership data on admin policy: {}",
                        err
                    );
                    Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None))
                }
            }
        }
        Ok(None) => Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None)),
        Err(err) => {
            error!(
                "Error checking guild membership data on admin policy: {}",
                err
            );
            Ok((Some(HttpResponse::Forbidden().body("Forbidden.")), None))
        }
    }
}
