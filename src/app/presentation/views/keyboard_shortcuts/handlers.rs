use axum::{extract::State, response::IntoResponse};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/keyboard_shortcuts/index.html");
    let context = context! {
        target => APP_ROUTES["kbshortcuts"].full_pathname(0),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn doit_hdlr() -> impl IntoResponse {
    "Did It!".into_response()
}
