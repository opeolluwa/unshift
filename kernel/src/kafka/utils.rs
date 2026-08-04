use std::time::Duration;

use rdkafka::{
    admin::{AdminClient, AdminOptions, NewTopic, TopicReplication},
    client::DefaultClientContext,
    consumer::{BaseConsumer, Consumer},
    producer::{FutureProducer, FutureRecord},
};

use crate::errors::AppError;

pub async fn retrieve_topics(
    client: &BaseConsumer,
) -> Result<Vec<String>, AppError> {
    let metadata = client
        .fetch_metadata(None, Duration::from_secs(10))
        .map_err(|err| AppError::KafkaError(err.to_string()))?;

    let topics = metadata
        .topics()
        .iter()
        .map(|t| t.name().to_string())
        .collect();

    Ok(topics)
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

pub async fn retrieve_topic(
    client: &BaseConsumer,
    topic_name: &str,
) -> Result<Vec<String>, AppError> {
    let metadata = client
        .fetch_metadata(Some(topic_name), Duration::from_secs(10))
        .map_err(|err| AppError::KafkaError(err.to_string()))?;

    let partitions: Vec<String> = metadata
        .topics()
        .iter()
        .flat_map(|t| t.partitions())
        .map(|p| format!("partition-{}-leader-{}", p.id(), p.leader()))
        .collect();

    Ok(partitions)
}
