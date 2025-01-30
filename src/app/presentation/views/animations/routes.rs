use axum::{routing::{delete, get, post}, Router};

use crate::app::{domain::entities::Path, TAppStateRouter, APP_ROUTES};

use super::handlers::*;

fn path(index: usize) -> Path {
    APP_ROUTES["animations"].get_path(index)
}

pub fn animations_routes() -> TAppStateRouter {
    Router::new()
        .route("/", get(root_hdlr))
        .route(&path(0).name,       get(throb_hdlr))
        .route(&path(0).variant(0), get(throb_colors_hdlr))
        .route(&path(1).name,       get(fadeout_hdlr))
        .route(&path(1).variant(0), delete(fadeout_demo_hdlr))
        .route(&path(2).name,       get(fadein_hdlr))
        .route(&path(2).variant(0), post(fadein_demo_hdlr))
        .route(&path(3).name,       get(requestin_hdlr))
        .route(&path(3).variant(0), post(requestin_name_hdlr))
        .route(&path(4).name,       get(usingclass_hdlr))
        .route(&path(5).name,       get(usingview_hdlr))
        .route(&path(5).variant(0), get(usingview_new_hdlr))
        .route(&path(5).variant(1), get(usingview_original_hdlr))
}
