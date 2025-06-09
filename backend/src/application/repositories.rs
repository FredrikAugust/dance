use anyhow::Result;

use crate::domain::{Company, Opportunity};

pub trait OpportunityRepo {
    async fn get_all(&self) -> Result<Vec<Opportunity>>;
    async fn save(&self, opportunity: &Opportunity) -> Result<()>;
}

pub trait CompanyRepo {
    async fn get_all(&self) -> Result<Vec<Company>>;
}
