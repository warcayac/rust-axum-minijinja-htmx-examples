use axum::{routing::{delete, get}, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn delete_row_routes() -> TAppStateRouter {
    let path = APP_ROUTES["deleterow"].get_path(0);

    Router::new()
        .route("/", get(root_hdlr))
        .route(&format!("{}/{{id}}", path.name), delete(delete_hdlr))
}
