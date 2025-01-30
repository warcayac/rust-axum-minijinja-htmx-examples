use axum::{extract::State, response::{IntoResponse, Redirect}, Form};
use minijinja::context;
use validator::Validate;

use crate::app::{domain::entities::Contact, presentation::helpers::*, AppError, TAppStateState, APP_ROUTES};


fn pathname_only(flag: bool) -> String {
    if flag {
        APP_ROUTES["click2edit"].full_pathname(0)
    } else {
        APP_ROUTES["click2edit"].full_pathname_variant(0, 0)
    }
}

pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/click_2_edit/index.html");
    let context = context! {
        target => pathname_only(true),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn view_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/click_2_edit/partials/contact_view.html");
    let context = context! {
        target => pathname_only(false),
        contact => state.contact.read().await.clone(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn edit_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/click_2_edit/partials/contact_edit.html");
    let context = context! {
        target => pathname_only(true), // put
        send_back => pathname_only(true), // get
        contact => state.contact.read().await.clone(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn update_hdlr(State(state): TAppStateState, Form(contact): Form<Contact>) -> impl IntoResponse {
    if let Err(e) = contact.validate() {
        return AppError::Validation(e).into_response();
    }
    
    state.contact.write().await.clone_from(&contact);

    Redirect::to(&pathname_only(true)).into_response()
}
