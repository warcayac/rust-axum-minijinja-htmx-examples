use axum::{http::StatusCode, response::{Html, IntoResponse, Response}};
use minijinja::{Value, Error, Template};


/// A wrapper type that we'll use to encapsulate HTML parsed by minijinja into valid HTML for axum to serve.
pub struct HtmlTemplate<'env, 'source>(pub Result<Template<'env, 'source>, Error>, pub Value);

impl IntoResponse for HtmlTemplate<'_, '_> {
    fn into_response(self) -> Response {
        match self.0 {
            // Attempt to render the template with MiniJinja
            Ok(template) => match template.render(self.1) {
                Ok(html) => Html(html).into_response(),
                // If we're not, return an error or some bit of fallback HTML
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to render template. Error: {e}"),
                ).into_response(),
            },
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get template. Error: {e}"),
            ).into_response(),
        }
    }
}

pub type TParamsMap<T> = std::collections::HashMap<String, T>;
