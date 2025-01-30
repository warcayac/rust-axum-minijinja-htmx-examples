use axum::{routing::get, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn move_before_routes() -> TAppStateRouter {
    let path = APP_ROUTES["movebefore"].pathname(0);

    Router::new()
        .route("/", get(root_hdlr))
        .route(&path, get(details_hdlr))
}
