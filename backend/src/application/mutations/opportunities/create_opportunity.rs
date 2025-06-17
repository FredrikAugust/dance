use crate::{application::repositories::OpportunityRepo, domain::opportunity::Opportunity};

pub async fn handle(
    repo: &impl OpportunityRepo,
    opportunity: &Opportunity,
) -> anyhow::Result<Opportunity> {
    repo.save(opportunity).await
}
