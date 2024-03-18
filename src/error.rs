//! Errors produced by the service.
//!
//! TODO: Add protocol level error handling.
use crate::transport::KafkaTransportError;
use std::fmt::{Display, Formatter};

/// Errors that can occur when calling Kafka.
#[derive(thiserror::Error, Debug)]
pub enum KafkaError {
    /// Kafka return an error in the response.
    Protocol,
    /// A transport level error occured trying to communicate with Kafka
    Transport(KafkaTransportError),
    /// An error occurred serializing or deserializing a message.
    Serde,
}

impl Display for KafkaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<anyhow::Error> for KafkaError {
    fn from(_: anyhow::Error) -> Self {
        KafkaError::Serde
    }
}

impl From<KafkaTransportError> for KafkaError {
    fn from(value: KafkaTransportError) -> Self {
        KafkaError::Transport(value)
    }
}
