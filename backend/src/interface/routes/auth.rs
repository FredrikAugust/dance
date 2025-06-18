use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{application::repositories::AuthRepo, interface::state::AppState};

#[derive(Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(credentials): Json<Credentials>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = app_state
        .authentication_service
        .authenticate_with_email_and_password(&credentials.email, &credentials.password)
        .await;

    result
        .map(Json)
        .map_err(|_| StatusCode::from_u16(401).unwrap())
}
