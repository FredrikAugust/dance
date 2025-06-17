use anyhow::Result;

use crate::domain::{company::Company, opportunity::Opportunity};

pub trait OpportunityRepo {
    async fn get_all(&self) -> Result<Vec<Opportunity>>;
    async fn save(&self, opportunity: &Opportunity) -> Result<Opportunity>;
}

pub trait CompanyRepo {
    async fn get_all(&self) -> Result<Vec<Company>>;
    async fn save(&self, company: &Company) -> Result<Company>;
}
