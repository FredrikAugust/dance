use chrono::DateTime;
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::user::UserId;

#[derive(Debug, Clone, Serialize, Deserialize, From, Into, Default, PartialEq, Eq, Hash)]
pub struct SessionId(pub Uuid);

#[derive(Debug, Deserialize, Clone)]
pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
    pub expires_at: DateTime<chrono::Utc>,
}

impl Session {
    pub fn new(user_id: UserId) -> Self {
        Self {
            id: SessionId(Uuid::new_v4()),
            user_id,
            expires_at: chrono::Utc::now() + chrono::Duration::days(14),
        }
    }
}

pub trait SessionService {
    fn create(&self, user_id: &UserId) -> anyhow::Result<Session>;

    fn get(&self, user_id: &SessionId) -> anyhow::Result<Option<Session>>;
}
