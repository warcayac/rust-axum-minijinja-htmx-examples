use std::{env::var, net::{IpAddr, SocketAddr}};

use eyre::Context;
use wshared::types::TResult;


#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn load() -> TResult<Self>  {
        dotenvy::dotenv()?;

        Ok(Self {
            host: var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080)
            ,
        })
    }

    pub fn socket_addr(&self) -> TResult<SocketAddr> {
        let host = self.host.parse::<IpAddr>().wrap_err("Invalid host in config")?;
        Ok(SocketAddr::from((host, self.port)))
    }
}