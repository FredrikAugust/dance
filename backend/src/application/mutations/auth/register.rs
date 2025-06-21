use crate::{
    application::{
        repositories::AuthRepo,
        services::{Session, SessionService},
    },
    domain,
};

pub async fn handle(
    authentication_service: &impl AuthRepo,
    session_store: &impl SessionService,
    email: &str,
    password: &str,
) -> anyhow::Result<(domain::user::User, Session)> {
    let user = authentication_service
        .register_with_email_and_password(email, password)
        .await?;

    let session = session_store.create(&user.id)?;

    Ok((user, session))
}
