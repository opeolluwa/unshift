use std::net::{Ipv4Addr, SocketAddrV4};

use axum::Router;
use kernel::{
    config::{env::load_env, logger::init_logger},
    errors::AppError,
    handlers::handle_404,
    kafka::connection::KafkaState,
    router,
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = load_env()?;

    init_logger(&config);

    let kafka_state = KafkaState::new()?;

    let app = Router::new()
        .nest("/api", router::routes(kafka_state))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .fallback(handle_404);

    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, config.port);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|err| AppError::StartupError(err.to_string()))?;

    tracing::info!("ignition started on http://{addr}");

    axum::serve(listener, app)
        .await
        .map_err(|err| AppError::StartupError(err.to_string()))?;

    Ok(())
}
