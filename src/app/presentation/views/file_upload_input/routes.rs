use axum::{routing::{get, post}, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn file_upload_input_routes() -> TAppStateRouter {
    let path = APP_ROUTES["fileuploadinput"].get_path(0);
    Router::new()
        .route("/", get(root_hdlr))
        .route(&path.name, post(error_hdlr))
}
