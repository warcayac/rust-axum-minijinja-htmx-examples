use axum::{extract::State, response::IntoResponse};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, TSharedAppState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/tabs_hateoas/index.html");
    let context = context! {
        target => format!("{}{}", APP_ROUTES["tabshateoas"].full_pathname(0), 1),
    };

    HtmlTemplate(template, context).into_response()
}

async fn generic_tab_hdlr(state: TSharedAppState, index: usize) -> impl IntoResponse {
    let template = state.env.get_template("pages/tabs_hateoas/partials/tab.html");
    let content = state.placeholders.read().await.get(index-1).unwrap().content.to_string();
    let context = context! {
        target => APP_ROUTES["tabshateoas"].full_pathname(0),
        content,
        active_tab => index,
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn tab1_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    generic_tab_hdlr(state, 1).await
}

pub async fn tab2_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    generic_tab_hdlr(state, 2).await
}

pub async fn tab3_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    generic_tab_hdlr(state, 3).await
}
