use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    adapters::{
        AddTopicRequest, ClusterOverviewResponse, CreateTopicResponse, Message, MessagesResponse,
        PublishMessageRequest, PublishResponse, TopicSummary, TopicsResponse,
    },
    errors::AppError,
    kafka::{connection::KafkaState, utils},
};

pub async fn health_check() -> impl IntoResponse {
    "healthy"
}

pub async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "the resource you are looking for does not not exist or has been removed",
    )
}

pub async fn get_topics(State(state): State<KafkaState>) -> Result<Json<TopicsResponse>, AppError> {
    let topics = utils::retrieve_topics(&state.metadata_client, &state.admin_client).await?;

    Ok(Json(TopicsResponse { topics }))
}

pub async fn get_cluster_overview(
    State(state): State<KafkaState>,
) -> Result<Json<ClusterOverviewResponse>, AppError> {
    let overview = utils::retrieve_cluster_overview(&state.metadata_client).await?;

    Ok(Json(overview))
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
) -> Result<Json<TopicSummary>, AppError> {
    let topic =
        utils::retrieve_topic(&state.metadata_client, &state.admin_client, &topic_name).await?;

    Ok(Json(topic))
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

#[derive(Debug, serde::Deserialize)]
pub struct GetMessagesParams {
    pub limit: Option<usize>,
}

pub async fn get_messages(
    State(state): State<KafkaState>,
    Path(topic_name): Path<String>,
    Query(params): Query<GetMessagesParams>,
) -> Result<Json<MessagesResponse>, AppError> {
    let limit = params.limit.unwrap_or(10).min(100);

    let messages: Vec<Message> =
        utils::retrieve_messages(&state.consumer, &topic_name, limit).await?;

    Ok(Json(MessagesResponse {
        topic: topic_name,
        messages,
    }))
}
