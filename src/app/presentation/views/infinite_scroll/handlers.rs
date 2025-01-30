use axum::{extract::{Query, State}, response::IntoResponse};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, APP_ROUTES};

use super::payloads::Payload;


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/infinite_scroll/index.html");
    let context = context! {
        target => APP_ROUTES["infinitescroll"].full_pathname(0),
        page => 1,
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn load_hdlr(State(state): TAppStateState, Query(payload): Query<Payload>) -> impl IntoResponse {
    let template = state.env.get_template("pages/infinite_scroll/partials/rows.html");
    let context = context! {
        target => APP_ROUTES["infinitescroll"].full_pathname(0),
        page => payload.page.unwrap_or(1),
    };

    HtmlTemplate(template, context).into_response()
}
