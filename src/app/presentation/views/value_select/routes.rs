use axum::{routing::get, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn value_select_routes() -> TAppStateRouter {
    let path = APP_ROUTES["valueselect"].get_path(0);

    Router::new()
        .route("/", get(root_hdlr))
        .route(&path.name, get(models_hdlr))
}
