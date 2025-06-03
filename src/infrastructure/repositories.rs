use std::str::FromStr;

use anyhow::Result;
use sqlx::{Executor, PgPool};
use uuid::Uuid;

use crate::{
    application::repositories::OpportunityRepo,
    domain::{CompanyId, Opportunity, OpportunityId},
};

pub struct SqlRepo {
    pub pool: PgPool,
}

impl OpportunityRepo for SqlRepo {
    async fn get_all(&self) -> Result<Vec<Opportunity>> {
        let results = sqlx::query!(r#"SELECT * FROM opportunities"#)
            .fetch_all(&self.pool)
            .await?;

        return Ok(results
            .iter()
            .map(|row| Opportunity {
                id: OpportunityId(row.id),
                company_id: CompanyId(row.company_id),
                location: row.location.clone(),
                start_time: row.start_time.and_utc(),
                end_time: row.end_time.and_then(|t| Some(t.and_utc())),
                image_urls: vec![],
                description: row.description.clone(),
                application_url: None,
            })
            .collect());
    }

    async fn save(&self, opportunity: &Opportunity) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO opportunities (id) VALUES ($1)"#,
            opportunity.id.0
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
