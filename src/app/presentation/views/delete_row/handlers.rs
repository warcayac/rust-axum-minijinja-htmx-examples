use axum::{extract::{Path, State}, response::IntoResponse};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/delete_row/index.html");
    let context = context! {
        target => APP_ROUTES["deleterow"].full_pathname(0),
        users => state.users1.read().await.clone(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn delete_hdlr(Path(_id): Path<u8>) -> impl IntoResponse {
    "".into_response()
}
