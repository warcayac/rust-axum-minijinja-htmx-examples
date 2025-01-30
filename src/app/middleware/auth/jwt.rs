use axum::{extract::FromRequestParts, http::request::Parts};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;

use crate::app::AppError;

use super::models::Claims;


pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = "JWT_SECRET".to_string();
    Keys::new(secret.as_bytes())
});

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the AUTH header
        let auth_header = parts
            .headers
            .get("AUTH")
            .ok_or(AppError::AuthMissingCredentials)?
        ;

        // Convert header to string
        let auth_str = auth_header.to_str().map_err(|_| AppError::AuthInvalidToken)?;

        // Decode and validate the token
        let token_data = decode::<Claims>(auth_str, &KEYS.decoding, &Validation::default())
            .map_err(|_| AppError::AuthInvalidToken)?
        ;

        Ok(token_data.claims)
    }
}
