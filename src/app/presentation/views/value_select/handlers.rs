use axum::{extract::{Query, State}, response::IntoResponse};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/value_select/index.html");
    let context = context! {
        target => APP_ROUTES["valueselect"].full_pathname(0),
        items => state.car_brands.read().await.iter().map(|item| item.name.clone()).collect::<Vec<String>>(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn models_hdlr(State(state): TAppStateState, Query(query): Query<TParamsMap<String>>) -> impl IntoResponse {
    let brand = query.get("make").unwrap();
    let template = state.env.get_template("pages/value_select/partials/options.html");
    let context = context! {
        items => state.car_brands.read().await
            .iter()
            .filter(|item| *item.name == *brand)
            .flat_map(|item| item.models.clone())
            .collect::<Vec<String>>()
        ,
    };

    HtmlTemplate(template, context).into_response()
}
