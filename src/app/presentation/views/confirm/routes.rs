use axum::{routing::get, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


fn path(idx: usize) -> String {
    APP_ROUTES["confirm"].pathname(idx)
}

pub fn confirm_routes() -> TAppStateRouter {
    Router::new()
        .route("/", get(root_hdlr))
        .route(&path(0), get(ex1_hdlr))
        .route(&path(1), get(ex2_hdlr))
        .route(&path(2), get(confirmed_hdlr))
}
