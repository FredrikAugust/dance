use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::{
    cookie::{Cookie, Expiration},
    CookieJar,
};
use serde::Deserialize;
use time::OffsetDateTime;

use crate::{
    application::{
        mutations::auth::{login, register},
        services::Session,
    },
    interface::{error::ApplicationError, state::AppState},
};

#[derive(Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

fn create_cookie_from_session<'a>(session: Session) -> anyhow::Result<Cookie<'a>> {
    let expiration = OffsetDateTime::from_unix_timestamp(session.expires_at.timestamp())?;

    let cookie = Cookie::build(("session_id", session.id.0.to_string()))
        .expires(expiration)
        .build();

    Ok(cookie)
}

pub async fn login(
    jar: CookieJar,
    State(app_state): State<AppState>,
    Json(credentials): Json<Credentials>,
) -> Result<(CookieJar, impl IntoResponse), ApplicationError> {
    let (user, session) = login::handle(
        &app_state.authentication_service,
        &app_state.session_store,
        &credentials.email,
        &credentials.password,
    )
    .await?;

    let cookie = create_cookie_from_session(session)?;

    let jar = jar.add(cookie);

    Ok((jar, Json(user)))
}

pub async fn register(
    jar: CookieJar,
    State(app_state): State<AppState>,
    Json(credentials): Json<Credentials>,
) -> Result<(CookieJar, impl IntoResponse), ApplicationError> {
    let (user, session) = register::handle(
        &app_state.authentication_service,
        &app_state.session_store,
        &credentials.email,
        &credentials.password,
    )
    .await?;

    let cookie = create_cookie_from_session(session)?;

    let jar = jar.add(cookie);

    Ok((jar, Json(user)))
}
