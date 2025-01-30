use axum::{routing::{get, post}, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn inline_validation_routes() -> TAppStateRouter {
    let path = APP_ROUTES["inlineval"].get_path(0);

    Router::new()
        .route("/", get(root_hdlr))
        .route(&format!("{}/email", path.name), post(update_hdlr))
}
