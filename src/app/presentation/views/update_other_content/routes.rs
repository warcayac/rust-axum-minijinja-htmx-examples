use axum::{routing::{get, post}, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


fn path(idx: usize, variant: Option<usize>) -> String { 
    match variant {
        Some(var_idx) => APP_ROUTES["updateother"].pathname_variant(idx, var_idx),
        None => APP_ROUTES["updateother"].pathname(idx),
    }
}

pub fn update_other_content_routes() -> TAppStateRouter {
    Router::new()
        .route("/", get(root_hdlr))
        .route(&path(0, None),    get(sol1_hdlr))
        .route(&path(0, Some(0)), post(sol1_newcontact_hdlr))
        .route(&path(1, None),    get(sol2_hdlr))
        .route(&path(1, Some(0)), post(sol2_newcontact_hdlr))
        .route(&path(2, None),    get(sol3_hdlr))
        .route(&path(2, Some(0)), post(sol3_newcontact_hdlr))
        .route(&path(2, Some(1)), get(sol3_contacts_hdlr))
        .route(&path(3, None),    get(sol4_hdlr))
        .route(&path(3, Some(0)), post(sol4_newcontact_hdlr))
        .route(&path(3, Some(1)), get(sol4_contacts_hdlr))
}
