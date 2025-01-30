use axum::{extract::State, response::IntoResponse, Form};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, APP_ROUTES};

use super::payloads::Payload;


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/active_search/index.html");
    let context = context! {
        target => APP_ROUTES["activesearch"].full_pathname(0),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn search_hdlr(State(state): TAppStateState, Form(payload): Form<Payload>) -> impl IntoResponse {
    let template = state.env.get_template("pages/active_search/partials/rows.html");
    let keyword = payload.search.to_lowercase();
    let mut users = state.users2.read().await.clone();
    if !payload.search.is_empty() {
        users = users
            .into_iter()
            .filter(|u| 
                u.first_name.to_lowercase().contains(&keyword) || 
                u.last_name.to_lowercase().contains(&keyword) ||
                u.email.to_lowercase().contains(&keyword)
            )
            .collect::<Vec<_>>()
        ;
    }
    let context = context! {
        users,
    };

    HtmlTemplate(template, context).into_response()
}
