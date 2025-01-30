use axum::{routing::get, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn lazy_load_routes() -> TAppStateRouter {
    let path = APP_ROUTES["lazyload"].get_path(0);

    Router::new()
        .route("/", get(root_hdlr))
        .route(&path.name, get(image_hdlr))
}
