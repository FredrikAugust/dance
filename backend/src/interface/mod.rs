use axum::routing::get;
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
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8084").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
