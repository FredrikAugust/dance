use crate::application::repositories::CompanyRepo;
use crate::domain::company::Company;
use anyhow::Result;

pub async fn handle(repo: &impl CompanyRepo, company: &Company) -> Result<Company> {
    repo.save(company).await
}
