use axum::routing::{get, post};
use log::info;
use sqlx::PgPool;

use crate::infrastructure::{
    auth::AuthenticationService, repositories::SqlRepo, services::SessionStore,
};

mod error;
mod routes;
mod state;

pub async fn run_web_server() -> anyhow::Result<()> {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Could not connect to database");

    let sql_repo = SqlRepo { pool };
    let app_state = state::AppState {
        repo: sql_repo.clone(),
        session_store: SessionStore {
            store: Default::default(),
        },
        authentication_service: AuthenticationService {
            sql_repo: sql_repo.clone(),
            salt: std::env::var("PASSWORD_SALT")?,
            argon2_config: argon2::Config::default(),
        },
    };

    let app = axum::Router::new()
        .route("/opportunities", get(routes::opportunities::get_all))
        .route("/opportunities", post(routes::opportunities::create))
        .route("/companies", get(routes::companies::get_all))
        .route("/companies", post(routes::companies::create))
        // Authentication
        .route("/login", post(routes::auth::login))
        .route("/register", post(routes::auth::register))
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
