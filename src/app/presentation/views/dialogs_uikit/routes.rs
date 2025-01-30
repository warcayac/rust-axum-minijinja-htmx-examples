use axum::{routing::get, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn dialogs_uikit_routes() -> TAppStateRouter {
    let path = APP_ROUTES["dialogsuikit"].get_path(0);

    Router::new()
        .route("/", get(root_hdlr))
        .route(&path.name, get(modal_hdlr))
}
