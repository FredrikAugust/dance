use chrono::{DateTime, Utc};
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, From, Into)]
pub struct CompanyId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: CompanyId,
    pub name: String,
    pub description: String,
    pub website_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, From, Into)]
pub struct OpportunityId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opportunity {
    pub id: OpportunityId,

    pub company_id: CompanyId,

    pub title: String,
    pub description: String,

    pub location: String,

    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,

    pub image_urls: Vec<String>,

    pub application_url: Option<String>,
    pub application_deadline: Option<DateTime<Utc>>,
}
