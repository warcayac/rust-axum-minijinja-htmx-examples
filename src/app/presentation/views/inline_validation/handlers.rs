use axum::{extract::State, response::IntoResponse, Form};
use minijinja::context;
use validator::ValidateEmail;

use crate::app::{domain::entities::Contact, presentation::helpers::*, TAppStateState, APP_ROUTES};


fn pathname() -> String { APP_ROUTES["inlineval"].full_pathname(0) }
fn tptname(s: &str) -> String { format!("pages/inline_validation/{s}") }

pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template(&tptname("index.html"));
    let context = context! {
        target => pathname(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn update_hdlr(State(state): TAppStateState, Form(contact): Form<Contact>) -> impl IntoResponse {
    let template = state.env.get_template(&tptname("partials/validate.html"));
    let context = context! {
        target => format!("{}/email", pathname()),
        email => contact.email.clone(),
        msg => match contact.email.validate_email() {
            true => match contact.email == "test@test.com" {
                true => "".to_string(),
                false => "That email is already taken. Please enter another email.".to_string(),
            },
            false => "Please enter a valid email address".to_string(),
        },
    };

    HtmlTemplate(template, context).into_response()
}
