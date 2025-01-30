use axum::{extract::State, response::IntoResponse};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/confirm/index.html");
    let context = context! {
        target1 => APP_ROUTES["confirm"].full_pathname(0),
        target2 => APP_ROUTES["confirm"].full_pathname(1),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn ex1_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/confirm/example1.html");
    let context = context! {
        target => APP_ROUTES["confirm"].full_pathname(2),
        back_menu => APP_ROUTES["confirm"].get_base(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn ex2_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/confirm/example2.html");
    let context = context! {
        target => APP_ROUTES["confirm"].full_pathname(2),
        back_menu => APP_ROUTES["confirm"].get_base(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn confirmed_hdlr() -> impl IntoResponse {
    "Confirmed from server!!".into_response()
}