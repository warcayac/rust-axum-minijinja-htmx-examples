use std::collections::HashMap;

use axum::{extract::State, http::{HeaderValue, StatusCode}, response::IntoResponse, Form};
use minijinja::context;

use crate::app::{domain::entities::{User1, User1Update}, presentation::helpers::*, TAppStateState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/update_other_content/index.html");
    let context = context! {
        targets => APP_ROUTES["updateother"].full_pathnames().collect::<Vec<_>>(),
        users => state.users1.read().await.clone(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn sol1_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/update_other_content/solution1.html");
    let context = context! {
        users => state.users1.read().await.clone(),
        target => APP_ROUTES["updateother"].full_pathname_variant(0,0),
        form_attribs => "hx-target=#contacts-table hx-swap=beforeend",
        back_menu => APP_ROUTES["updateother"].get_base(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn sol1_newcontact_hdlr(State(state): TAppStateState, Form(user): Form<User1Update>) -> impl IntoResponse {
    let template = state.env.get_template("pages/update_other_content/partials/user_row_1.html");
    let context = context! {
        user,
    };

    (StatusCode::CREATED, HtmlTemplate(template, context)).into_response()
}

pub async fn sol2_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/update_other_content/solution2.html");
    let context = context! {
        users => state.users1.read().await.clone(),
        target => APP_ROUTES["updateother"].full_pathname_variant(1,0),
        form_attribs => "hx-swap=none",
        back_menu => APP_ROUTES["updateother"].get_base(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn sol2_newcontact_hdlr(State(state): TAppStateState, Form(user): Form<User1Update>) -> impl IntoResponse {
    let template = state.env.get_template("pages/update_other_content/partials/user_row_2.html");
    let context = context! {
        user,
    };

    (StatusCode::CREATED, HtmlTemplate(template, context)).into_response()
}

pub async fn sol3_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let mut tbody_attribs = HashMap::new();
    let path_get= APP_ROUTES["updateother"].full_pathname_variant(2,1);

    tbody_attribs.insert("hx-get", path_get.as_str());
    tbody_attribs.insert("hx-trigger", "newContact from:body");

    let template = state.env.get_template("pages/update_other_content/solution3.html");
    let context = context! {
        users => state.users1.read().await.clone(),
        target => APP_ROUTES["updateother"].full_pathname_variant(2,0),
        tbody_attribs,
        form_attribs => "hx-swap=none",
        back_menu => APP_ROUTES["updateother"].get_base(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn sol3_newcontact_hdlr(State(state): TAppStateState, Form(user): Form<User1Update>) -> impl IntoResponse {
    let mut users = state.users1.write().await;
    let new_user = User1 {
        id: (users.len() + 1) as u8,
        name: user.name.unwrap(),
        email: user.email.unwrap(),
        active: user.active.unwrap_or(true),
    };
    users.push(new_user);

    let mut response = (StatusCode::CREATED, "").into_response();
    // let mut response = axum::response::Response::new("".to_string());
    response.headers_mut().insert("hx-trigger", HeaderValue::from_static("newContact"));
    // *response.status_mut() = StatusCode::CREATED;

    response
}

pub async fn sol3_contacts_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/update_other_content/partials/rows_1.html");
    let context = context! {
        users => state.users1.read().await.clone(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn sol4_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let mut tbody_attribs = HashMap::new();
    let target = APP_ROUTES["updateother"].full_pathname_variant(3,0);
    let path_get= APP_ROUTES["updateother"].full_pathname_variant(3,1);

    /*
    EXPLICACIÓN: cuando el evento "path-deps" se produzca, producto de una petición no-GET 
    (es decir: POST, PUT, DELETE, PATCH) a la ruta "/contacts", entonces se realizará una
    petición GET a la ruta "/contacts/table" y se actualizará el contenido del elemento
    contenedor de estos atributos.
    Para que esto funcione, la biblioteca "path-deps.js" debe declarse antes de uso o referencia,
    el atributo hx-ext="path-deps" debe declararse en un elemento contenedor de los elementos
    que se verán afectados por el evento "path-deps", en nuestro caso serían: Contacts y NewContact.
    No hay necesidad de establecer alguna cabecera especial a la respuesta de la solicitud no-GET.
    */
    tbody_attribs.insert("hx-get", path_get.as_str());
    tbody_attribs.insert("hx-trigger", "path-deps");
    tbody_attribs.insert("path-deps", target.as_str());
    tbody_attribs.insert("hx-swap", "innerhtml");

    let template = state.env.get_template("pages/update_other_content/solution4.html");
    let context = context! {
        users => state.users1.read().await.clone(),
        target,
        tbody_attribs,
        form_attribs => "hx-swap=none",
        back_menu => APP_ROUTES["updateother"].get_base(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn sol4_newcontact_hdlr(State(state): TAppStateState, Form(user): Form<User1Update>) -> impl IntoResponse {
    let mut users = state.users1.write().await;
    let new_user = User1 {
        id: (users.len() + 1) as u8,
        name: user.name.unwrap(),
        email: user.email.unwrap(),
        active: user.active.unwrap_or(true),
    };
    users.push(new_user);

    (StatusCode::CREATED, "").into_response()
}

pub async fn sol4_contacts_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/update_other_content/partials/rows_1.html");
    let context = context! {
        users => state.users1.read().await.clone(),
    };

    HtmlTemplate(template, context).into_response()
}
