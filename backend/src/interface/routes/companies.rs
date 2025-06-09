use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{application::queries::companies::get_all_companies, interface::state::AppState};

pub async fn get_all(State(app_state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    match get_all_companies::handle(&app_state.repo).await {
        Ok(companies) => Ok(Json(companies)),
        Err(_e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
