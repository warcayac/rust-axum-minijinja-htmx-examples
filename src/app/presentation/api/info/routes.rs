use axum::{routing::get, Router};

use crate::app::TAppStateRouter;

use super::handlers::health_check;


pub fn info_routes() -> TAppStateRouter {
    Router::new()
        .route("/health-check", get(health_check))
}