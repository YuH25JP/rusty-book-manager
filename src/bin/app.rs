use std::net::{Ipv4Addr, SocketAddr};

use adapter::database::connect_database_with;
use anyhow::{Error, Result};
use api::route::health::build_health_check_routers;
use axum::Router;
use registry::AppRegistry;
use shared::config::AppConfig;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap().await
}

async fn bootstrap() -> Result<()> {
    // create AppConfig object
    let app_config = AppConfig::new()?;
    // establish database connection
    let pool = connect_database_with(&app_config.database);

    // create AppRegistry object
    let app_reg = AppRegistry::new(pool);

    let app = Router::new()
        .merge(build_health_check_routers())
        .with_state(app_reg);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app).await.map_err(Error::from)
}
