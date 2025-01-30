use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;


#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Resource not found")]
    NotFound,
    #[error("Validation error")]
    Validation(#[from] validator::ValidationErrors),
    #[error("Internal server error")]
    Internal(#[from] eyre::Error),

    #[error("Wrong credentials")]
    AuthWrongCredentials,
    #[error("Missing credentials")]
    AuthMissingCredentials,
    // #[error("Token creation error")]
    // AuthTokenCreation,
    #[error("Invalid token")]
    AuthInvalidToken,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<Vec<FieldError>>,
}

#[derive(Serialize)]
struct FieldError {
    field: String,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, details) = match self {
            // AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string(), None),
            AppError::Validation(ref errors) => {
                let details = errors
                    .field_errors()
                    .into_iter()
                    .map(|(field, errors)| FieldError {
                        field: field.to_string(),
                        message: errors
                            .iter()
                            .map(|e| e.message.as_ref().map(|m| m.to_string()).unwrap_or_default())
                            .collect::<Vec<_>>()
                            .join(", ")
                        ,
                    })
                    .collect::<Vec<_>>()
                ;
                (StatusCode::BAD_REQUEST, self.to_string(), Some(details))
            },
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), None),
            
            AppError::AuthWrongCredentials => (StatusCode::UNAUTHORIZED, self.to_string(), None),
            AppError::AuthMissingCredentials => (StatusCode::BAD_REQUEST, self.to_string(), None),
            // AppError::AuthTokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), None),
            AppError::AuthInvalidToken => (StatusCode::BAD_REQUEST, self.to_string(), None),
        };

        let body = Json(ErrorResponse { 
            error: error_message,
            details,
        });

        (status, body).into_response()
    }
}