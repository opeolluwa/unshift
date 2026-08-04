use crate::{
    handlers::{
        create_topic, extract_json, extract_multiple_path, extract_multiple_types, extract_path,
        extract_query, get_topic, get_topics, handle_404, health_check, publish_message,
    },
    kafka::connection::KafkaState,
};
use axum::{
    Router,
    routing::{get, post, put},
};

pub fn routes(state: KafkaState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/user/{user_identifier}", get(extract_path))
        .route(
            "/workspace/{workspace_identifie}/user/{user_identifier}",
            get(extract_multiple_path),
        )
        .route("/login", post(extract_query))
        .route("/profile", put(extract_json))
        .route("/profile/{id}", post(extract_multiple_types))
        .route("/topics", get(get_topics).post(create_topic))
        .route("/topics/{topic_name}", get(get_topic))
        .route("/topics/{topic_name}/messages", post(publish_message))
        .fallback(handle_404)
        .with_state(state)
}
