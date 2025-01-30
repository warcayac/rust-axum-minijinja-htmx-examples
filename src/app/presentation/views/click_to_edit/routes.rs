use axum::{routing::{get, put}, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn click_to_edit_routes() -> TAppStateRouter {
    let path = APP_ROUTES["click2edit"].get_path(0);

    Router::new()
        .route("/", get(root_hdlr))
        .route(&path.name, get(view_hdlr))
        .route(&path.variant(0), get(edit_hdlr))
        .route(&path.name, put(update_hdlr))
}
