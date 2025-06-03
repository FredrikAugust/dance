use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    application::queries::opportunities::get_all_opportunities, interface::state::AppState,
};

pub async fn get_all(State(app_state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    match get_all_opportunities::handle(&app_state.repo).await {
        Ok(opportunities) => Ok(Json(opportunities)),
        Err(_e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
