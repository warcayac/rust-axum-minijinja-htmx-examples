use axum::{extract::{RawForm, State}, response::IntoResponse};
use minijinja::context;

use crate::app::{presentation::{helpers::*, views::sortable::payloads::Payload}, TAppStateState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/sortable/index.html");
    let context = context! {
        target => APP_ROUTES["sortable"].full_pathname(0),
        items => vec![1, 2, 3, 4, 5],
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn items_hdlr(State(state): TAppStateState, raw: RawForm) -> impl IntoResponse {
    let payload = Payload::from_raw(raw);
    let template = state.env.get_template("pages/sortable/partials/items.html");
    let context = context! {
        items => payload.items,
    };

    HtmlTemplate(template, context).into_response()
}
