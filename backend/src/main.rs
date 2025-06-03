use anyhow::Result;

mod application;
mod domain;
mod infrastructure;
mod interface;

#[tokio::main]
async fn main() -> Result<()> {
    interface::run_web_server().await?;

    Ok(())
}
