use crate::domain::user::{User, UserId};
use anyhow::{Context, Result};
use argon2::Config;
use uuid::Uuid;

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
        password: &str,
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

        let existing_hash = user.password_hash.clone();

        let matches = argon2::verify_encoded(&existing_hash, password.as_bytes())
            .context("could not compare hashes")?;

        if matches {
            Ok(user)
        } else {
            Err(anyhow::anyhow!("password does not match"))
        }
    }

    async fn register_with_email_and_password(
        &self,
        email: &str,
        password: &str,
    ) -> Result<crate::domain::user::User> {
        let user = crate::domain::user::User {
            id: UserId(Uuid::new_v4()),
            email: email.to_string(),
            password_hash: argon2::hash_encoded(
                password.as_bytes(),
                self.salt.as_bytes(),
                &self.argon2_config,
            )
            .context("hashing password")?,
        };

        sqlx::query!(
            "INSERT INTO users (id, email, password_hash) VALUES ($1, $2, $3)",
            Uuid::new_v4(),
            user.email,
            user.password_hash
        )
        .execute(&self.sql_repo.pool)
        .await
        .context("inserting user")?;

        Ok(user)
    }
}
