use axum::{routing::{get, post}, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn bulk_update_routes() -> TAppStateRouter {
    let path = APP_ROUTES["bulkupdate"].get_path(0);

    Router::new()
        .route("/", get(root_hdlr))
        .route(&path.name, post(update_hdlr))
}
