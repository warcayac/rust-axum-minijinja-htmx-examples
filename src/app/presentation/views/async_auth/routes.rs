use axum::{routing::{get, post}, Router};

use crate::app::{TAppStateRouter, APP_ROUTES};

use super::handlers::*;


pub fn async_auth_routes() -> TAppStateRouter {
    let example = APP_ROUTES["asyncauth"].pathname(0);
    let token = APP_ROUTES["asyncauth"].pathname_variant(1, 0);
    
    Router::new()
        .route("/", get(root_hdlr))
        .route(&example, post(example_hdlr))
        .route(&token, post(token_hdlr))
}
