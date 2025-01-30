use std::time::Duration;

use axum::{extract::State, response::{Html, IntoResponse}};
use minijinja::context;
use tokio::time::sleep;

use crate::app::{presentation::helpers::*, TAppStateState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/animations/index.html");
    let context = context! {
        target1 => APP_ROUTES["animations"].full_pathname(0),
        target2 => APP_ROUTES["progressbar"].get_base(),
        target3 => APP_ROUTES["animations"].full_pathname(1),
        target4 => APP_ROUTES["animations"].full_pathname(2),
        target5 => APP_ROUTES["animations"].full_pathname(3),
        target6 => APP_ROUTES["animations"].full_pathname(4),
        target7 => APP_ROUTES["animations"].full_pathname(5),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn throb_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/animations/throb.html");
    let context = context! {
        target => APP_ROUTES["animations"].full_pathname_variant(0, 0),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn throb_colors_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/animations/partials/throb_colors.html");
    let context = context! {
        target => APP_ROUTES["animations"].full_pathname_variant(0, 0),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn fadeout_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/animations/fadeout.html");
    let context = context! {
        target => APP_ROUTES["animations"].full_pathname_variant(1, 0),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn fadeout_demo_hdlr() -> impl IntoResponse {
    Html("")
}

pub async fn fadein_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/animations/fadein.html");
    let context = context! {
        target => APP_ROUTES["animations"].full_pathname_variant(2, 0),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn fadein_demo_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/animations/partials/fadein_demo.html");
    let context = context! {
        target => APP_ROUTES["animations"].full_pathname_variant(2, 0),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn requestin_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/animations/requestin.html");
    let context = context! {
        target => APP_ROUTES["animations"].full_pathname_variant(3, 0),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn requestin_name_hdlr() -> impl IntoResponse {
    let _ = sleep(Duration::from_millis(450)).await;
    Html("Submitted!")
}

pub async fn usingclass_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/animations/usingclass.html");
    let context = context! {
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn usingview_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/animations/usingview.html");
    let context = context! {
        target => APP_ROUTES["animations"].full_pathname_variant(5, 0),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn usingview_new_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/animations/partials/usingview_new.html");
    let context = context! {
        target => APP_ROUTES["animations"].full_pathname_variant(5, 1),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn usingview_original_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/animations/partials/usingview_original.html");
    let context = context! {
        target => APP_ROUTES["animations"].full_pathname_variant(5, 0),
    };

    HtmlTemplate(template, context).into_response()
}

