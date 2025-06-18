use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, From, Into, Default)]
pub struct UserId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,

    pub email: String,

    pub password_hash: String,
}

/// Ensure we do not leak any private information
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.email)
            .field("password", &"[REDACTED]")
            .finish()
    }
}
