use crate::{
    handlers::{create_topic, get_topic, get_topics, health_check, publish_message},
    kafka::connection::KafkaState,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes(state: KafkaState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/topics", get(get_topics).post(create_topic))
        .route("/topics/{topic_name}", get(get_topic))
        .route("/topics/{topic_name}/messages", post(publish_message))
        .with_state(state)
}
