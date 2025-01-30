use axum::{extract::State, response::IntoResponse};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/file_upload_input/index.html");
    let context = context! {
        target => APP_ROUTES["fileuploadinput"].full_pathname(0),
        show_error => false,
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn error_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/file_upload_input/partials/form.html");
    let context = context! {
        target => APP_ROUTES["fileuploadinput"].full_pathname(0),
        show_error => true,
    };

    HtmlTemplate(template, context).into_response()
}
