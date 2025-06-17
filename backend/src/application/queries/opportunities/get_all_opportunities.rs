use crate::{application::repositories::OpportunityRepo, domain::opportunity::Opportunity};

pub async fn handle(repo: &impl OpportunityRepo) -> anyhow::Result<Vec<Opportunity>> {
    repo.get_all().await
}
