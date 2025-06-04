use crate::{
    application::repositories::OpportunityRepo,
    domain::{Opportunity, OpportunityId},
};
use anyhow::Result;
use sqlx::PgPool;
use url::Url;

#[derive(Clone)]
pub struct SqlRepo {
    pub pool: PgPool,
}

impl OpportunityRepo for SqlRepo {
    async fn get_all(&self) -> Result<Vec<Opportunity>> {
        let results = sqlx::query!(r#"SELECT * FROM opportunities"#)
            .fetch_all(&self.pool)
            .await?;

        Ok(results
            .iter()
            .map(|row| Opportunity {
                id: OpportunityId(row.id),
                location: row.location.clone(),
                start_time: row.start_time.and_utc(),
                end_time: row.end_time.map(|t| t.and_utc()),
                image_urls: vec![],
                description: row.description.clone(),
                application_url: row
                    .application_url
                    .as_ref()
                    .map(|url| Url::parse(url).unwrap()),
            })
            .collect())
    }

    async fn save(&self, opportunity: &Opportunity) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO opportunities (id, location, start_time, end_time, image_urls, description, application_url) VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
            opportunity.id.0,
            opportunity.location,
            opportunity.start_time.naive_utc(),
            opportunity.end_time.map(|t| t.naive_utc()),
            &opportunity.image_urls.iter().map(Url::to_string).collect::<Vec<String>>(),
            opportunity.description,
            opportunity.application_url.as_ref().map(|url| url.to_string())
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
