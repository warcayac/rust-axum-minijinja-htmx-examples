use axum::{extract::State, response::IntoResponse};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/move_before/index.html");
    let context = context! {
        target => APP_ROUTES["movebefore"].full_pathname(0),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn details_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/move_before/details.html");
    let context = context! {
    };

    HtmlTemplate(template, context).into_response()
}
