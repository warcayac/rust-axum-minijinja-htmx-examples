use axum::response::IntoResponse;
use rand::Rng;

use super::AppError;


pub async fn not_found() -> impl IntoResponse {
    AppError::NotFound
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await
        ;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("👋 shutdown signal received, starting graceful shutdown..."); 
}

pub fn generate_id(length: Option<usize>) -> Result<String, minijinja::Error> {
    let length = length.unwrap_or(15);
    let characters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let mut id = String::with_capacity(length);

    for _ in 0..length {
        let random_index = rand::thread_rng().gen_range(0..characters.len());
        id.push(characters[random_index]);
    }

    Ok(id)
}

pub fn generate_random_color() -> String {
    let colors =["orange", "blue", "green", "red", "purple", "yellow", "pink", "brown"];
    let index = rand::thread_rng().gen_range(0..colors.len());
    colors[index].to_string()
}
