use anyhow::Result;
use application::repositories::OpportunityRepo;
use chrono::Utc;
use domain::{CompanyId, Opportunity};
use infrastructure::repositories::SqlRepo;
use sqlx::PgPool;
use uuid::Uuid;

mod application;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let repo = SqlRepo { pool };

    repo.save(&Opportunity::new(
        CompanyId(Uuid::new_v4()),
        "San Francisco".to_string(),
        Utc::now(),
        None,
        vec![],
        "Test".to_string(),
        None,
    ))
    .await?;
    let opportunities_repo = repo.get_all().await.expect("Failed to get opportunities");
    println!("{:?}", opportunities_repo);

    Ok(())
}
