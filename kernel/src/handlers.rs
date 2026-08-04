use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    adapters::{
        AddTopicRequest, CreateTopicResponse, PublishMessageRequest, PublishResponse,
        TopicResponse, TopicsResponse, User, Pagination,
    },
    errors::AppError,
    kafka::{connection::KafkaState, utils},
};

pub async fn health_check() -> impl IntoResponse {
    "healthy"
}

// path variables workspace/:workspace_id/user/:id
// workspace/
// workspace/:id
// workspace/:id/user/:userId
pub async fn extract_path(Path(user_identifier): Path<String>) -> impl IntoResponse {
    user_identifier
}

pub async fn extract_multiple_path(
    Path((workspace_identifier, user_identifier)): Path<(String, String)>,
) -> impl IntoResponse {
    format!("{user_identifier}\n{workspace_identifier}")
}

// /search?username=&password=password
pub async fn extract_query(Query(user): Query<User>) -> impl IntoResponse {
    format!("username: {}\npassword:{}", user.username, user.password)
}

pub async fn extract_json(Json(user): Json<User>) -> impl IntoResponse {
    format!("username: {}\npassword:{}", user.username, user.password)
}

// profile/:id?pageSize=5&currentPage=3
pub async fn extract_multiple_types(
    Query(pagination): Query<Pagination>,
    Path(user_identifier): Path<String>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    dbg!("{}", user);
    dbg!("{}", pagination);
    dbg!("{}", user_identifier);

    "response"
}

pub async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "the resource you are looking for does not not exist or has been removed",
    )
}

pub async fn get_topics(
    State(state): State<KafkaState>,
) -> Result<Json<TopicsResponse>, AppError> {
    let topics = utils::retrieve_topics(&state.metadata_client).await?;

    Ok(Json(TopicsResponse { topics }))
}

pub async fn create_topic(
    State(state): State<KafkaState>,
    Json(body): Json<AddTopicRequest>,
) -> Result<Json<CreateTopicResponse>, AppError> {
    utils::add_topic(
        &state.admin_client,
        &body.name,
        body.num_partitions,
        body.replication_factor,
    )
    .await?;

    Ok(Json(CreateTopicResponse {
        name: body.name,
        status: "created".to_string(),
    }))
}

pub async fn get_topic(
    State(state): State<KafkaState>,
    Path(topic_name): Path<String>,
) -> Result<Json<TopicResponse>, AppError> {
    let partitions = utils::retrieve_topic(&state.metadata_client, &topic_name).await?;

    Ok(Json(TopicResponse {
        topic: topic_name,
        partitions,
    }))
}

pub async fn publish_message(
    State(state): State<KafkaState>,
    Path(topic_name): Path<String>,
    Json(body): Json<PublishMessageRequest>,
) -> Result<Json<PublishResponse>, AppError> {
    utils::publish_to_topic(&state.producer, &topic_name, &body.key, &body.payload).await?;

    Ok(Json(PublishResponse {
        topic: topic_name,
        status: "published".to_string(),
    }))
}
