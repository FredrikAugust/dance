use crate::{application::repositories::OpportunityRepo, domain::Opportunity};

pub async fn handle(
    repo: &impl OpportunityRepo,
    opportunity: &Opportunity,
) -> anyhow::Result<Opportunity> {
    repo.save(opportunity).await
}
