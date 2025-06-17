use crate::{application::repositories::CompanyRepo, domain::company::Company};
use anyhow::Result;

pub async fn handle(repo: &impl CompanyRepo) -> Result<Vec<Company>> {
    repo.get_all().await
}
