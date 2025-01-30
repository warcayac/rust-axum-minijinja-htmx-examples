use axum::{routing::get, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn infinite_scroll_routes() -> TAppStateRouter {
    let path = APP_ROUTES["infinitescroll"].get_path(0);

    Router::new()
        .route("/", get(root_hdlr))
        .route(&path.name, get(load_hdlr))
}
