use axum::{routing::get, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn web_components_routes() -> TAppStateRouter {
    let path = APP_ROUTES["webcomponents"].pathname(0);
    
    Router::new()
        .route("/", get(root_hdlr))
        .route(&path, get(from_component_hdlr))
}
