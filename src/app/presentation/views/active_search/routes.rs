use axum::{routing::{get, post}, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn active_search_routes() -> TAppStateRouter {
    let path = APP_ROUTES["activesearch"].get_path(0);
    Router::new()
        .route("/", get(root_hdlr))
        .route(&path.name, post(search_hdlr))
}
