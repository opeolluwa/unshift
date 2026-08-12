use std::sync::Arc;

use rdkafka::{
    ClientConfig,
    admin::AdminClient,
    client::DefaultClientContext,
    consumer::{BaseConsumer, DefaultConsumerContext},
    producer::FutureProducer,
};

use crate::errors::AppError;

fn create_client_config() -> Result<ClientConfig, AppError> {
    let broker = std::env::var("KAFKA_BROKER")
        .map_err(|err| AppError::EnvExtractionError("KAFKA_BROKER".into(), err.to_string()))?;

    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", &broker);

    Ok(config)
}

pub fn create_producer() -> Result<FutureProducer, AppError> {
    let mut config = create_client_config()?;
    config
        .set("message.timeout.ms", "5000")
        .set("queue.buffering.max.messages", "100000");

    config
        .create()
        .map_err(|err| AppError::KafkaError(err.to_string()))
}

pub fn create_metadata_client() -> Result<BaseConsumer<DefaultConsumerContext>, AppError> {
    let config = create_client_config()?;

    config
        .create()
        .map_err(|err| AppError::KafkaError(err.to_string()))
}

pub fn create_consumer() -> Result<BaseConsumer<DefaultConsumerContext>, AppError> {
    let mut config = create_client_config()?;
    config
        .set("group.id", "unshift-console")
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "false");

    config
        .create()
        .map_err(|err| AppError::KafkaError(err.to_string()))
}

pub fn create_admin_client() -> Result<AdminClient<DefaultClientContext>, AppError> {
    let config = create_client_config()?;

    config
        .create()
        .map_err(|err| AppError::KafkaError(err.to_string()))
}

#[derive(Clone)]
pub struct KafkaState {
    pub producer: FutureProducer,
    pub metadata_client: Arc<BaseConsumer<DefaultConsumerContext>>,
    pub admin_client: Arc<AdminClient<DefaultClientContext>>,
}

impl KafkaState {
    pub fn new() -> Result<Self, AppError> {
        let producer = create_producer()?;
        let metadata_client = Arc::new(create_metadata_client()?);
        let admin_client = Arc::new(create_admin_client()?);

        Ok(Self {
            producer,
            metadata_client,
            admin_client,
        })
    }
}
