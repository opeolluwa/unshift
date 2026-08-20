use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct AddTopicRequest {
    pub name: String,
    pub num_partitions: i32,
    pub replication_factor: i32,
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct PublishMessageRequest {
    pub key: String,
    pub payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<PublishMessageHeader>>,
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct PublishMessageHeader {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct TopicsResponse {
    pub topics: Vec<TopicSummary>,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TopicSummary {
    pub topic: String,
    pub partitions: i32,
    pub preferred_leader_percent: i32,
    pub under_replicated: i32,
    pub custom_configs: i32,
    pub configs: Vec<TopicConfig>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TopicConfig {
    pub name: String,
    pub value: Option<String>,
    pub read_only: bool,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct CreateTopicResponse {
    pub name: String,
    pub status: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct PublishResponse {
    pub topic: String,
    pub status: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct Message {
    pub key: Option<String>,
    pub payload: String,
    pub partition: i32,
    pub offset: i64,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct MessagesResponse {
    pub topic: String,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ClusterOverviewResponse {
    pub bootstrap_servers: String,
    pub total_topics: usize,
    pub total_partitions: usize,
    pub preferred_partition_leader_percentage: f64,
    pub total_under_replicated_partitions: usize,
}
