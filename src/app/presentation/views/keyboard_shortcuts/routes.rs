use axum::{routing::{get, post}, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn keyboard_shortcuts_routes() -> TAppStateRouter {
    let path = APP_ROUTES["kbshortcuts"].pathname(0);

    Router::new()
        .route("/", get(root_hdlr))
        .route(&path, post(doit_hdlr))
}
