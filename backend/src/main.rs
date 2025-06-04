use anyhow::Result;
use log::info;

mod application;
mod domain;
mod infrastructure;
mod interface;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("Starting dance-dance-revolution backend");

    interface::run_web_server().await?;

    Ok(())
}
