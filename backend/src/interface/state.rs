use crate::infrastructure::repositories::SqlRepo;

#[derive(Clone)]
pub struct AppState {
    pub repo: SqlRepo,
}
