use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    application::queries::companies::{create_company, get_all_companies},
    domain::{Company, CompanyId},
    interface::state::AppState,
};

pub async fn get_all(State(app_state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    match get_all_companies::handle(&app_state.repo).await {
        Ok(companies) => Ok(Json(companies)),
        Err(_e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct CreateCompanyRequest {
    name: String,
    description: String,
    website_url: Option<String>,
}

pub async fn create(
    State(app_state): State<AppState>,
    Json(company): Json<CreateCompanyRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match create_company::handle(
        &app_state.repo,
        &Company {
            id: CompanyId(Uuid::new_v4()),
            name: company.name,
            description: company.description,
            website_url: company.website_url,
        },
    )
    .await
    {
        Ok(company) => Ok(Json(company)),
        Err(_e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
