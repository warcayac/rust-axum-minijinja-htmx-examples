use axum::{extract::State, http::StatusCode, response::{Html, IntoResponse}, Form};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, APP_ROUTES};

use super::payloads::Payload;


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/reset_user_input/index.html");
    let context = context! {
        target => APP_ROUTES["resetuserinput"].full_pathname(0),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn note_hdlr(Form(payload): Form<Payload>) -> impl IntoResponse {
    match payload.note.is_empty() {
        true => Html("".to_string()).into_response(),
        false => match payload.note.to_lowercase().contains("error") {
            true => (StatusCode::BAD_REQUEST, "You requested an error!").into_response(),
            false => Html(format!("<li>{}</li>", payload.note)).into_response(),
        }
    }
}