use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpportunityId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: CompanyId,
    pub name: String,
    pub location: String,
    pub opportunities: Vec<Opportunity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opportunity {
    pub id: OpportunityId,
    pub company_id: CompanyId,
    pub location: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub image_urls: Vec<Url>,
    pub description: String,
    pub application_url: Option<Url>,
}

impl Opportunity {
    pub fn new(
        company_id: CompanyId,
        location: String,
        start_time: DateTime<Utc>,
        end_time: Option<DateTime<Utc>>,
        image_urls: Vec<Url>,
        description: String,
        application_url: Option<Url>,
    ) -> Self {
        Self {
            id: OpportunityId(Uuid::new_v4()),
            company_id,
            location,
            start_time,
            end_time,
            image_urls,
            description,
            application_url,
        }
    }
}
