use crate::{
    handlers::{
        create_topic, get_cluster_overview, get_messages, get_topic, get_topics, health_check,
        publish_message,
    },
    kafka::connection::KafkaState,
};
use axum::{Router, routing::get};

pub fn routes(state: KafkaState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/cluster/overview", get(get_cluster_overview))
        .route("/topics", get(get_topics).post(create_topic))
        .route("/topics/{topic_name}", get(get_topic))
        .route(
            "/topics/{topic_name}/messages",
            get(get_messages).post(publish_message),
        )
        .with_state(state)
}
