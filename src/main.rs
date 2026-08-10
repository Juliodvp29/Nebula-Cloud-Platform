mod api;
mod app;
mod audit;
mod auth;
mod compute;
mod config;
mod error;
mod iam;
mod jobs;
mod lambda;
mod platform;
mod s3;

use std::net::SocketAddr;

use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("nebula=debug,tower_http=info")),
        )
        .init();

    let config = config::Config::from_env()?;

    let address = SocketAddr::new(config.host.parse()?, config.port);

    let state = app::AppState::new(config.clone());

    let router = app::router(state);

    info!(
        host = %config.host,
        port = config.port,
        "Nebula API starting"
    );

    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, router).await?;

    Ok(())
}
