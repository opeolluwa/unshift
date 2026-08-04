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
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct TopicsResponse {
    pub topics: Vec<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct TopicResponse {
    pub topic: String,
    pub partitions: Vec<String>,
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
