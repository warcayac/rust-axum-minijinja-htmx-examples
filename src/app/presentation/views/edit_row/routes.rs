use axum::{routing::{get, put}, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn edit_row_routes() -> TAppStateRouter {
    let path = APP_ROUTES["editrow"].get_path(0);

    Router::new()
        .route("/", get(root_hdlr))
        .route(&format!("{}/{{id}}", path.name), get(view_user_hdlr))
        .route(&format!("{}/{{id}}/edit", path.name), get(edit_user_hdlr))
        .route(&format!("{}/{{id}}", path.name), put(update_user_hdlr))
}
