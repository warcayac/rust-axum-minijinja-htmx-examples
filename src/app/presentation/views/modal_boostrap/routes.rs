use axum::{routing::get, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn modal_boostrap_routes() -> TAppStateRouter {
    let path = APP_ROUTES["modalbootstrap"].get_path(0);

    Router::new()
        .route("/", get(root_hdlr))
        .route(&path.name, get(modal_hdlr))
}
