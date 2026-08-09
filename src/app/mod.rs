use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{api, config::Config};

#[derive(Clone)]
pub struct AppState {
    config: Arc<Config>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(api::health::health))
        .with_state(state)
}
