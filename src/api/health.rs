use axum::{extract::State, response::IntoResponse};

use crate::app::AppState;

pub async fn health(State(state): State<AppState>) -> impl IntoResponse {
    let _config = state.config();

    "OK"
}
