use axum::{extract::{Path, State}, response::IntoResponse, Form};
use minijinja::context;
use validator::Validate;

use crate::app::{domain::entities::*, presentation::helpers::*, AppError, TAppStateState, APP_ROUTES};


fn pathname() -> String { APP_ROUTES["editrow"].full_pathname(0) }
fn tptname(s: &str) -> String { format!("pages/edit_row/{s}") }

pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template(&tptname("index.html"));
    let context = context! {
        target => pathname(),
        users => state.users1.read().await.clone(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn view_user_hdlr(State(state): TAppStateState, Path(id): Path<u8>) -> impl IntoResponse {
    let template = state.env.get_template(&tptname("partials/row.html"));
    let context = context! {
        target => pathname(),
        user => state.users1.read().await.iter().find(|u| u.id == id).unwrap().clone(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn edit_user_hdlr(State(state): TAppStateState, Path(id): Path<u8>) -> impl IntoResponse {
    let template = state.env.get_template(&tptname("partials/edit.html"));
    let context = context! {
        target => pathname(),
        user => state.users1.read().await.iter().find(|u| u.id == id).unwrap().clone(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn update_user_hdlr(
    State(state): TAppStateState, 
    Path(id): Path<u8>, 
    Form(payload): Form<User1Update>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return AppError::Validation(e).into_response();
    }

    // Scope the write lock to only the update section so we avoid the deadlock
    {
        let mut users = state.users1.write().await;
        if let Some(user) = users.iter_mut().find(|u| u.id == id) {
            user.update(payload);
        }
    }

    let template = state.env.get_template(&tptname("partials/row.html"));
    let context = context! {
        target => pathname(),
        user => state.users1.read().await.iter().find(|u| u.id == id).unwrap_or(&User1::default()).clone(),
    };

    HtmlTemplate(template, context).into_response()
}
