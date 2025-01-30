use axum::{routing::{get, post}, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn sortable_routes() -> TAppStateRouter {
    let path = APP_ROUTES["sortable"].pathname(0);
    
    Router::new()
        .route("/", get(root_hdlr))
        .route(&path, post(items_hdlr))
}
