use axum::{extract::State, response::IntoResponse};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/dialogs_uikit/index.html");
    let context = context! {
        target => APP_ROUTES["dialogsuikit"].full_pathname(0),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn modal_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/dialogs_uikit/partials/modal.html");
    let context = context! {
    };

    HtmlTemplate(template, context).into_response()
}
