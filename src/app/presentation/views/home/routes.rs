use axum::{routing::get, Router};

use crate::app::TAppStateRouter;

use super::handlers::*;


pub fn home_routes() -> TAppStateRouter {
    Router::new()
        .route("/", get(root_hdlr))
}
