use wshared::types::TResult;

use super::{config::Config, state::{AppState, TSharedAppState}};


async fn connect_to_database(_config: &Config) -> TResult<()> {
    Ok(())
}

pub async fn init() -> TResult<(TSharedAppState, std::net::SocketAddr)> {
    let config = Config::load()?;
    
    connect_to_database(&config).await?;

    let socket_addr = config.socket_addr()?;

    Ok((AppState::new().await?, socket_addr))
}
