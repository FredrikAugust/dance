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

    async fn save(&self, opportunity: &Opportunity) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO opportunities (id, company_id, title, description, location, start_time, end_time, image_urls, application_url, application_deadline) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
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
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

impl CompanyRepo for SqlRepo {
    async fn get_all(&self) -> Result<Vec<Company>> {
        let results = sqlx::query_as!(Company, "SELECT * FROM companies")
            .fetch_all(&self.pool)
            .await?;

        Ok(results)
    }
}
