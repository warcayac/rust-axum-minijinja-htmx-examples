mod app;

use axum::{http::{header::*, Method}, middleware::from_fn};
use tower::ServiceBuilder;

use waxum_logger::{init_logger, log_middleware, WLoggerConfig};
use wshared::types::TVoidResult;


#[tokio::main]
async fn main() -> TVoidResult {
    // initialize the logger with default config
    init_logger(WLoggerConfig::default());

    let (state, addr) = app::init().await?;
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| {
            tracing::error!("🚨 Failed to bind to address: {}", e);
            eyre::eyre!(e)
        })?
    ;

    tracing::info!("🚀 Axum server is listening on {}", addr);

    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(tower_http::cors::Any)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
    ;

    let app = app::routes()
        .nest_service("/assets", axum::routing::get_service(tower_http::services::ServeDir::new("public")))
        .with_state(state)
        .layer(from_fn(log_middleware))
        .layer(ServiceBuilder::new().layer(cors))
        .fallback(app::not_found)
    ;

    axum::serve(listener, app)
        .with_graceful_shutdown(app::shutdown_signal())
        .await
        .map_err(|e| {
            tracing::error!("💢 Error serving application: {e}");
            eyre::eyre!(e)
        })?
    ;

    Ok(())
}
