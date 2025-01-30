use axum::{extract::{Query, State}, response::IntoResponse};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, APP_ROUTES};

use super::payloads::Payload;


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/click_2_load/index.html");
    let context = context! {
        page => 1,
        target => APP_ROUTES["click2load"].full_pathname(0),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn load_hdlr(State(state): TAppStateState, Query(payload): Query<Payload>) -> impl IntoResponse {
    let template = state.env.get_template("pages/click_2_load/partials/rows.html");
    let context = context! {
        page => payload.page.unwrap_or(1),
        target => APP_ROUTES["click2load"].full_pathname(0),
    };

    HtmlTemplate(template, context).into_response()
}
