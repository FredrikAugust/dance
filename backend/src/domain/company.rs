use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, From, Into, Default)]
pub struct CompanyId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: CompanyId,
    pub name: String,
    pub description: String,
    pub website_url: Option<String>,
}
