use crate::{
    db::Db,
    handlers::{
        create_topic, delete_saved_message, get_cluster_overview, get_messages, get_topic,
        get_topics, health_check, list_saved_messages, publish_message, save_message,
    },
    kafka::connection::KafkaState,
};
use axum::{Router, routing::{delete, get}};

pub fn routes(kafka_state: KafkaState, db: Db) -> Router {
    let kafka_routes = Router::new()
        .route("/health", get(health_check))
        .route("/cluster/overview", get(get_cluster_overview))
        .route("/topics", get(get_topics).post(create_topic))
        .route("/topics/{topic_name}", get(get_topic))
        .route(
            "/topics/{topic_name}/messages",
            get(get_messages).post(publish_message),
        )
        .with_state(kafka_state);

    let db_routes = Router::new()
        .route("/saved-messages", get(list_saved_messages).post(save_message))
        .route("/saved-messages/{id}", delete(delete_saved_message))
        .with_state(db);

    kafka_routes.merge(db_routes)
}
