use axum::{extract::State, http::HeaderMap, response::{Html, IntoResponse}};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/dialogs/index.html");
    let context = context! {
        target => APP_ROUTES["dialogs"].full_pathname(0),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn submit_hdlr(headers: HeaderMap) -> impl IntoResponse {
    let value = headers.get("hx-prompt").unwrap().to_str().unwrap_or("");
    Html(format!("User entered <i>{}</i>", value))
}
