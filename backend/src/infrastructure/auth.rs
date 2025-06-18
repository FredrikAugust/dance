use crate::domain::user::User;
use anyhow::{Context, Result};
use argon2::Config;

use crate::application::repositories::AuthRepo;

use super::repositories::SqlRepo;

#[derive(Clone)]
pub struct AuthenticationService {
    pub sql_repo: SqlRepo,

    pub salt: String,
    pub argon2_config: Config<'static>,
}

impl AuthRepo for AuthenticationService {
    async fn authenticate_with_email_and_password(
        &self,
        email: &str,
        _password: &str,
    ) -> Result<crate::domain::user::User> {
        let user_result = sqlx::query_as!(User, "SELECT * FROM users WHERE email=$1", email)
            .fetch_optional(&self.sql_repo.pool)
            .await;

        let Ok(Some(user)) = user_result else {
            match user_result {
                Ok(Some(_)) => unreachable!(),
                Ok(None) => return Err(anyhow::anyhow!("user not found")),
                Err(err) => return Err(err.into()),
            }
        };

        let incoming_password_hash = argon2::hash_encoded(
            user.password_hash.clone().as_bytes(),
            self.salt.as_bytes(),
            &self.argon2_config,
        )
        .context("hashing password")?;

        let existing_hash = user.password_hash.clone();

        let matches = argon2::verify_encoded(&existing_hash, incoming_password_hash.as_bytes())
            .context("could not compare hashes")?;

        if matches {
            Ok(user)
        } else {
            Err(anyhow::anyhow!("password does not match"))
        }
    }

    async fn get_user_with_id(
        &self,
        _id: &crate::domain::user::UserId,
    ) -> Result<Option<crate::domain::user::User>> {
        todo!()
    }
}
