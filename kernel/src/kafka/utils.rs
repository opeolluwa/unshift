use std::{collections::HashMap, time::Duration};

use rdkafka::{
    admin::{
        AdminClient, AdminOptions, ConfigSource, NewTopic, OwnedResourceSpecifier,
        ResourceSpecifier, TopicReplication,
    },
    client::DefaultClientContext,
    consumer::{BaseConsumer, Consumer},
    metadata::MetadataTopic,
    producer::{FutureProducer, FutureRecord},
};

use crate::{
    adapters::{ClusterOverviewResponse, TopicConfig, TopicSummary},
    errors::AppError,
};

pub async fn retrieve_topics(
    client: &BaseConsumer,
    admin_client: &AdminClient<DefaultClientContext>,
) -> Result<Vec<TopicSummary>, AppError> {
    let metadata = client
        .fetch_metadata(None, Duration::from_secs(10))
        .map_err(|err| AppError::KafkaError(err.to_string()))?;

    let names: Vec<String> = metadata
        .topics()
        .iter()
        .map(|topic| topic.name().to_string())
        .collect();

    let configs = retrieve_topic_configs(admin_client, &names).await?;

    let topics = metadata
        .topics()
        .iter()
        .map(|topic| {
            let topic_configs = configs.get(topic.name()).cloned().unwrap_or_default();
            build_topic_summary(topic, &topic_configs)
        })
        .collect();

    Ok(topics)
}

pub async fn retrieve_cluster_overview(
    client: &BaseConsumer,
) -> Result<ClusterOverviewResponse, AppError> {
    let metadata = client
        .fetch_metadata(None, Duration::from_secs(10))
        .map_err(|err| AppError::KafkaError(err.to_string()))?;

    let mut total_partitions = 0usize;
    let mut preferred_leaders = 0usize;
    let mut under_replicated = 0usize;
    let mut total_topics = 0usize;

    for topic in metadata.topics() {
        if topic.name().contains("__") {
            continue;
        }

        total_topics += 1;

        for partition in topic.partitions() {
            total_partitions += 1;

            if partition.replicas().first().copied() == Some(partition.leader()) {
                preferred_leaders += 1;
            }

            if partition.isr().len() < partition.replicas().len() {
                under_replicated += 1;
            }
        }
    }

    let preferred_percentage = if total_partitions > 0 {
        ((preferred_leaders as f64 / total_partitions as f64) * 100.0 * 100.0).round() / 100.0
    } else {
        0.0
    };

    let bootstrap_servers =
        std::env::var("KAFKA_BROKER").unwrap_or_else(|_| "localhost:9092".to_string());

    Ok(ClusterOverviewResponse {
        bootstrap_servers,
        total_topics,
        total_partitions,
        preferred_partition_leader_percentage: preferred_percentage,
        total_under_replicated_partitions: under_replicated,
    })
}

pub async fn retrieve_topic(
    client: &BaseConsumer,
    admin_client: &AdminClient<DefaultClientContext>,
    topic_name: &str,
) -> Result<TopicSummary, AppError> {
    let metadata = client
        .fetch_metadata(Some(topic_name), Duration::from_secs(10))
        .map_err(|err| AppError::KafkaError(err.to_string()))?;

    let names: Vec<String> = metadata
        .topics()
        .iter()
        .map(|topic| topic.name().to_string())
        .collect();

    let configs = retrieve_topic_configs(admin_client, &names).await?;

    metadata
        .topics()
        .iter()
        .find(|topic| topic.name() == topic_name)
        .map(|topic| {
            let topic_configs = configs.get(topic_name).cloned().unwrap_or_default();
            build_topic_summary(topic, &topic_configs)
        })
        .ok_or_else(|| AppError::KafkaError(format!("topic {topic_name} not found")))
}

fn build_topic_summary(topic: &MetadataTopic, configs: &[TopicConfig]) -> TopicSummary {
    let partitions = topic.partitions();
    let total = partitions.len() as i32;

    let preferred = partitions
        .iter()
        .filter(|partition| partition.replicas().first().copied() == Some(partition.leader()))
        .count() as i32;

    let preferred_leader_percent = if total > 0 {
        ((preferred as f64 / total as f64) * 100.0).round() as i32
    } else {
        0
    };

    let under_replicated = partitions
        .iter()
        .filter(|partition| partition.isr().len() < partition.replicas().len())
        .count() as i32;

    TopicSummary {
        topic: topic.name().to_string(),
        partitions: total,
        preferred_leader_percent,
        under_replicated,
        custom_configs: configs.len() as i32,
        configs: configs.to_vec(),
    }
}

async fn retrieve_topic_configs(
    admin_client: &AdminClient<DefaultClientContext>,
    names: &[String],
) -> Result<HashMap<String, Vec<TopicConfig>>, AppError> {
    if names.is_empty() {
        return Ok(HashMap::new());
    }

    let resources: Vec<ResourceSpecifier<'_>> = names
        .iter()
        .map(|name| ResourceSpecifier::Topic(name))
        .collect();

    let options = AdminOptions::new().operation_timeout(Some(Duration::from_secs(10)));

    let results = admin_client
        .describe_configs(&resources, &options)
        .await
        .map_err(|err| AppError::KafkaAdminError(err.to_string()))?;

    let mut configs = HashMap::new();

    for result in results {
        let Ok(resource) = result else { continue };

        let name = match &resource.specifier {
            OwnedResourceSpecifier::Topic(name) => name,
            _ => continue,
        };

        let entries = resource
            .entries
            .into_iter()
            .filter(|entry| entry.source == ConfigSource::DynamicTopic)
            .map(|entry| TopicConfig {
                name: entry.name,
                value: entry.value,
                read_only: entry.is_read_only,
            })
            .collect();

        configs.insert(name.clone(), entries);
    }

    Ok(configs)
}

pub async fn publish_to_topic(
    producer: &FutureProducer,
    topic: &str,
    key: &str,
    payload: &str,
) -> Result<(), AppError> {
    let record = FutureRecord::to(topic).key(key).payload(payload);

    producer
        .send(record, Duration::from_secs(5))
        .await
        .map_err(|(err, _)| AppError::KafkaError(err.to_string()))?;

    Ok(())
}

pub async fn add_topic(
    admin_client: &AdminClient<DefaultClientContext>,
    topic_name: &str,
    num_partitions: i32,
    replication_factor: i32,
) -> Result<(), AppError> {
    let new_topic = NewTopic::new(
        topic_name,
        num_partitions,
        TopicReplication::Fixed(replication_factor),
    );

    let options = AdminOptions::new().operation_timeout(Some(Duration::from_secs(10)));

    let results = admin_client
        .create_topics([new_topic].iter(), &options)
        .await
        .map_err(|err| AppError::KafkaAdminError(err.to_string()))?;

    for result in results {
        result.map_err(|(name, code)| AppError::KafkaAdminError(format!("{name}: {code}")))?;
    }

    Ok(())
}
