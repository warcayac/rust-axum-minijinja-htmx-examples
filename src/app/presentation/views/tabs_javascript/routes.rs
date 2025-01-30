use axum::{routing::get, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


fn tab_path(i: u8) -> String {
    format!("{}{i}", APP_ROUTES["tabshateoas"].pathname(0))
}

pub fn tabs_javascript_routes() -> TAppStateRouter {
    Router::new()
        .route("/", get(root_hdlr))
        .route(&tab_path(1), get(tab1_hdlr))
        .route(&tab_path(2), get(tab2_hdlr))
        .route(&tab_path(3), get(tab3_hdlr))
}
