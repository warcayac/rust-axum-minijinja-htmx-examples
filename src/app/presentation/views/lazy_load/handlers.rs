use axum::{extract::State, response::{Html, IntoResponse}};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/lazy_load/index.html");
    let context = context! {
        target => APP_ROUTES["lazyload"].full_pathname(0),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn image_hdlr() -> impl IntoResponse {
    Html("<img src=\"/assets/img/tokyo.png\" alt=\"Tokyo Climate\"/>")
}
