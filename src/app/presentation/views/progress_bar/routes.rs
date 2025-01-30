use axum::{routing::{get, post}, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn progress_bar_routes() -> TAppStateRouter {
    let path_start = APP_ROUTES["progressbar"].get_path(1);
    let path_job = APP_ROUTES["progressbar"].get_path(0);

    Router::new()
        .route("/", get(root_hdlr))
        .route(&path_start.name, post(start_hdlr))
        .route(&path_job.variant(0), get(progress_hdlr))
        .route(&path_job.name, get(done_hdlr))
}
