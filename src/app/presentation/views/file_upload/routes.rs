use axum::{extract::DefaultBodyLimit, routing::{get, post}, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


fn path(index: usize) -> String {
    APP_ROUTES["fileupload"].get_path(index).name
}

pub fn file_upload_routes() -> TAppStateRouter {
    Router::new()
        .route("/", get(root_hdlr))
        .route(&path(0), get(js_hdlr))
        .route(&path(1), get(hs_hdlr))
        .route(&path(2), post(upload_hdlr)).layer(DefaultBodyLimit::max(1024 * 1024 * 500)) // max: 500 MB
}
