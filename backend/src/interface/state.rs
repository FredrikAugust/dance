use crate::infrastructure::{
    auth::AuthenticationService, repositories::SqlRepo, services::SessionStore,
};

#[derive(Clone)]
pub struct AppState {
    pub repo: SqlRepo,
    pub session_store: SessionStore,
    pub authentication_service: AuthenticationService,
}
