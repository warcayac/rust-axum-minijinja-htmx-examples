use axum::{extract::{Query, State}, response::{Html, IntoResponse}};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/web_components/index.html");
    let context = context! {
        target => APP_ROUTES["webcomponents"].full_pathname(0),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn from_component_hdlr(Query(params): Query<TParamsMap<usize>>) -> impl IntoResponse {
    let counter = params.get("counter").unwrap_or(&0);
    let plural = if *counter <= 1 { "" } else { "s" };
    Html(format!("<p>Clicked {counter} time{plural}!</p>"))
}
