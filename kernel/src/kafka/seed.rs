use std::path::Path;

use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TopicSeedFile {
    topics: Vec<TopicSeed>,
}

#[derive(Debug, Deserialize)]
struct TopicSeed {
    name: String,
    #[serde(default = "default_partitions")]
    partitions: i32,
    #[serde(default = "default_replication_factor")]
    replication_factor: i32,
}

fn default_partitions() -> i32 {
    1
}

fn default_replication_factor() -> i32 {
    1
}

pub async fn seed_topics(admin_client: &AdminClient<DefaultClientContext>, path: &Path) {
    if !path.exists() {
        tracing::debug!("no topics file at {}, skipping seed", path.display());
        return;
    }

    let contents = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(err) => {
            tracing::warn!("failed to read topics file {}: {err}", path.display());
            return;
        }
    };

    let seed_file: TopicSeedFile = match serde_yaml::from_str(&contents) {
        Ok(f) => f,
        Err(err) => {
            tracing::warn!("failed to parse topics file {}: {err}", path.display());
            return;
        }
    };

    if seed_file.topics.is_empty() {
        return;
    }

    let new_topics: Vec<NewTopic<'_>> = seed_file
        .topics
        .iter()
        .map(|t| {
            NewTopic::new(
                &t.name,
                t.partitions,
                TopicReplication::Fixed(t.replication_factor),
            )
        })
        .collect();

    let options = AdminOptions::new().operation_timeout(Some(std::time::Duration::from_secs(10)));

    match admin_client
        .create_topics(new_topics.iter(), &options)
        .await
    {
        Ok(results) => {
            for result in results {
                match result {
                    Ok(name) => tracing::info!("seeded topic: {name}"),
                    Err((name, code)) => {
                        if code == rdkafka::types::RDKafkaErrorCode::TopicAlreadyExists {
                            tracing::debug!("topic already exists: {name}");
                        } else {
                            tracing::warn!("failed to seed topic {name}: {code}");
                        }
                    }
                }
            }
        }
        Err(err) => {
            tracing::warn!("failed to create seeded topics: {err}");
        }
    }
}
