use std::collections::HashMap;

use axum::{extract::State, response::IntoResponse, Form};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/bulk_update/index.html");
    let context = context! {
        target => APP_ROUTES["bulkupdate"].full_pathname(0),
        users => state.users1.read().await.clone(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn update_hdlr(State(state): TAppStateState, Form(data): Form<HashMap<String, String>>) -> impl IntoResponse {
    let checked_emails = data.into_keys().map(|k| k.split(':').last().unwrap().to_string()).collect::<Vec<_>>();
    let mut users1 = state.users1.write().await;
    let (mut active, mut inactive) = (0, 0);

    for user in users1.iter_mut() {
        let old_value = user.active;
        
        user.active = checked_emails.contains(&user.email);
        
        if old_value != user.active {
            if user.active {
                active += 1;
            } else {
                inactive += 1;
            }
        }
    }
   

    let template = state.env.get_template("pages/bulk_update/partials/toast.html");
    let context = context! {
        active,
        inactive,
    };

    HtmlTemplate(template, context).into_response()
}
