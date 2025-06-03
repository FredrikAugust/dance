use anyhow::Result;

use crate::domain::Opportunity;

pub trait OpportunityRepo {
    async fn get_all(&self) -> Result<Vec<Opportunity>>;
    async fn save(&self, opportunity: &Opportunity) -> Result<()>;
}
