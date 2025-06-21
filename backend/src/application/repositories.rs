use anyhow::Result;

use crate::domain::{company::Company, opportunity::Opportunity, user::User};

pub trait OpportunityRepo {
    async fn get_all(&self) -> Result<Vec<Opportunity>>;
    async fn save(&self, opportunity: &Opportunity) -> Result<Opportunity>;
}

pub trait CompanyRepo {
    async fn get_all(&self) -> Result<Vec<Company>>;
    async fn save(&self, company: &Company) -> Result<Company>;
}

pub trait AuthRepo {
    async fn authenticate_with_email_and_password(
        &self,
        email: &str,
        password: &str,
    ) -> Result<User>;
    async fn register_with_email_and_password(&self, email: &str, password: &str) -> Result<User>;
}
