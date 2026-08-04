#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("failed to start up the app due to {0}")]
    StartupError(String),
    #[error("invalid env was passed {0}")]
    InvalidEnv(String),
    #[error("failed to extract {0} due to {1}")]
    EnvExtractionError(String, String),
    #[error("failed to parse {0} due to {1}")]
    EnvParsingError(String, String),
    #[error("kafka error: {0}")]
    KafkaError(String),
    #[error("kafka admin error: {0}")]
    KafkaAdminError(String),
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match &self {
            AppError::KafkaError(_) | AppError::KafkaAdminError(_) => {
                axum::http::StatusCode::BAD_GATEWAY
            }
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, axum::Json(ErrorResponse { message: self.to_string() })).into_response()
    }
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    message: String,
}
