use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::application::services::{Session, SessionId, SessionService};

#[derive(Clone)]
pub struct SessionStore {
    pub store: Arc<Mutex<HashMap<SessionId, Session>>>,
}

impl SessionService for SessionStore {
    fn create(&self, user_id: &crate::domain::user::UserId) -> anyhow::Result<Session> {
        let mut store = self.store.lock().expect("could not acquire lock");

        let session = Session::new(user_id.clone());
        store.insert(session.id.clone(), session.clone());

        Ok(session)
    }

    fn get(
        &self,
        session_id: &SessionId,
    ) -> anyhow::Result<Option<crate::application::services::Session>> {
        let store = self.store.lock().expect("could not acquire lock");

        let session = store.get(session_id);

        Ok(session.cloned())
    }
}
