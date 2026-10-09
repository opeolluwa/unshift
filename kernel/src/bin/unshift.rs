use std::net::{Ipv4Addr, SocketAddrV4};
use tower_http::services::{ServeDir, ServeFile};

use axum::{Router, http::StatusCode};
use kernel::{
    config::{cors::init_cors, env::load_env, logger::init_logger},
    db::Db,
    errors::AppError,
    kafka::{connection::KafkaState, seed},
    router,
};
use tower_http::timeout::TimeoutLayer;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = load_env()?;

    init_logger(&config);

    let kafka_state = KafkaState::new()?;

    seed::seed_topics(&kafka_state.admin_client, &config.topics_file).await;

    let db = Db::open(&config.data_dir)?;

    let cors = init_cors(&config);

    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

    let app = Router::new()
        .nest("/api", router::routes(kafka_state, db))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            config.requests_time_out_secs,
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(cors)
        .fallback_service(serve_dir);

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
