use axum::{extract::State, {http::StatusCode, response::IntoResponse}};

use crate::app::TAppStateState;


pub async fn health_check(State(_state): TAppStateState) -> impl IntoResponse {
    (StatusCode::OK, "Service is healthy")
}
