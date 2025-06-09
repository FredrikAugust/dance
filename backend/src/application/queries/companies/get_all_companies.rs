use crate::application::repositories::CompanyRepo;
use crate::domain::Company;
use anyhow::Result;

pub async fn handle(repo: &impl CompanyRepo) -> Result<Vec<Company>> {
    repo.get_all().await
}
