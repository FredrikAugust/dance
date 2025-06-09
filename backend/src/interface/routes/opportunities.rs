use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    application::{
        mutations::opportunities::create_opportunity, queries::opportunities::get_all_opportunities,
    },
    domain::{CompanyId, Opportunity, OpportunityId},
    interface::state::AppState,
};

pub async fn get_all(State(app_state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    match get_all_opportunities::handle(&app_state.repo).await {
        Ok(opportunities) => Ok(Json(opportunities)),
        Err(_e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct CreateOpportunityRequest {
    company_id: CompanyId,
    title: String,
    description: String,
    location: String,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
    image_urls: Vec<String>,
    application_url: Option<String>,
    application_deadline: Option<DateTime<Utc>>,
}

pub async fn create(
    State(app_state): State<AppState>,
    Json(company): Json<CreateOpportunityRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match create_opportunity::handle(
        &app_state.repo,
        &Opportunity {
            id: OpportunityId(Uuid::new_v4()),
            company_id: company.company_id,
            title: company.title,
            description: company.description,
            location: company.location,
            start_time: company.start_time,
            end_time: company.end_time,
            image_urls: company.image_urls,
            application_url: company.application_url,
            application_deadline: company.application_deadline,
        },
    )
    .await
    {
        Ok(company) => Ok(Json(company)),
        Err(_e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
