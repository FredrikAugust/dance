use crate::infrastructure::{auth::AuthenticationService, repositories::SqlRepo};

#[derive(Clone)]
pub struct AppState {
    pub repo: SqlRepo,
    pub authentication_service: AuthenticationService,
}
