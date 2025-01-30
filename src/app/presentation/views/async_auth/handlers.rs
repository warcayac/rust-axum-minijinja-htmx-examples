use axum::{extract::State, response::{Html, IntoResponse}, Json};
use jsonwebtoken::{encode, Header};
use minijinja::context;

use crate::app::{auth::{AuthRequest, AuthResponse, Claims, KEYS}, presentation::helpers::*, AppError, TAppStateState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/async_auth/index.html");
    let context = context! {
        target => APP_ROUTES["asyncauth"].full_pathname(0),
        token_path => APP_ROUTES["asyncauth"].full_pathname_variant(1, 0),
    };

    HtmlTemplate(template, context).into_response()
}

// protected endpoint: /example
pub async fn example_hdlr(claims: Claims) -> impl IntoResponse {
    Html(format!(
        "<p>Protected content for user: {} from company: {}</p>",
        claims.sub, claims.company
    ))
}

// generate a JWT token: /auth/token
pub async fn token_hdlr(Json(credentials): Json<AuthRequest>) -> impl IntoResponse {
    if credentials.client_id != "valid_id" || credentials.client_secret != "client_secret" {
        return AppError::AuthWrongCredentials.into_response();
    }

    let claims = Claims {
        sub: "warcayac".to_string(),
        company: "ACME".to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
    };

    match encode(&Header::default(), &claims, &KEYS.encoding) {
        Ok(token) => Json(AuthResponse { token }).into_response(),
        Err(_) => AppError::AuthInvalidToken.into_response(),
    }
}
