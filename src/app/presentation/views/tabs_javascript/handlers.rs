use axum::{extract::State, response::{Html, IntoResponse}};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, TSharedAppState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/tabs_javascript/index.html");
    let context = context! {
        target => APP_ROUTES["tabsjavascript"].full_pathname(0),
        active_tab => 1,
    };

    HtmlTemplate(template, context).into_response()
}

async fn generic_tab_hdlr(state: TSharedAppState, index: usize) -> impl IntoResponse {
    let content = state.placeholders.read().await.get(index).unwrap().content.to_string();
    Html(format!("<p>{}</p>", content))
}

pub async fn tab1_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    generic_tab_hdlr(state, 0).await
}

pub async fn tab2_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    generic_tab_hdlr(state, 1).await
}

pub async fn tab3_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    generic_tab_hdlr(state, 2).await
}
