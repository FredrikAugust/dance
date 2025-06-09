use crate::{
    application::repositories::{CompanyRepo, OpportunityRepo},
    domain::{Company, Opportunity},
};
use anyhow::Result;
use sqlx::PgPool;

#[derive(Clone)]
pub struct SqlRepo {
    pub pool: PgPool,
}

impl OpportunityRepo for SqlRepo {
    async fn get_all(&self) -> Result<Vec<Opportunity>> {
        let results = sqlx::query_as!(Opportunity, "SELECT * FROM opportunities")
            .fetch_all(&self.pool)
            .await?;

        Ok(results)
    }

    async fn save(&self, opportunity: &Opportunity) -> Result<Opportunity> {
        let result = sqlx::query_as!(Opportunity,
            r#"INSERT INTO opportunities (id, company_id, title, description, location, start_time, end_time, image_urls, application_url, application_deadline) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *"#,
            opportunity.id.0,
            opportunity.company_id.0,
            opportunity.title,
            opportunity.description,
            opportunity.location,
            opportunity.start_time,
            opportunity.end_time,
            &opportunity.image_urls,
            opportunity.application_url.as_ref().map(|url| url.to_string()),
            opportunity.application_deadline
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }
}

impl CompanyRepo for SqlRepo {
    async fn get_all(&self) -> Result<Vec<Company>> {
        let results = sqlx::query_as!(Company, "SELECT * FROM companies")
            .fetch_all(&self.pool)
            .await?;

        Ok(results)
    }

    async fn save(&self, company: &Company) -> Result<Company> {
        let result = sqlx::query_as!(Company, "INSERT INTO companies (id, name, description, website_url) VALUES ($1, $2, $3, $4) RETURNING *",
            company.id.0,
            company.name,
            company.description,
            company.website_url
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }
}
