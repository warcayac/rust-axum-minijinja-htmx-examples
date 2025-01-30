use axum::{extract::State, response::IntoResponse};
use minijinja::context;

use crate::app::{domain::entities::Toc, presentation::helpers::*, TAppStateState};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/home.html");
    let toc = Toc::new().await.unwrap().content;
    let context = context! {
        toc => toc,
    };

    HtmlTemplate(template, context).into_response()
}
