use axum::routing::{get, post};
use log::info;
use sqlx::PgPool;

use crate::infrastructure::repositories::SqlRepo;

mod routes;
mod state;

pub async fn run_web_server() -> anyhow::Result<()> {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Could not connect to database");

    let app_state = state::AppState {
        repo: SqlRepo { pool },
    };

    let app = axum::Router::new()
        .route("/opportunities", get(routes::opportunities::get_all))
        .route("/opportunities", post(routes::opportunities::create))
        .route("/companies", get(routes::companies::get_all))
        .route("/companies", post(routes::companies::create))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(format!(
        "0.0.0.0:{}",
        std::env::var("PORT").unwrap_or("8084".to_string())
    ))
    .await?;

    info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await?;

    Ok(())
}
