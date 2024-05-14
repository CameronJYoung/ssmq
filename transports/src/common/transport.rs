use async_trait::async_trait;

use crate::common::error::TransportError;

#[async_trait]
pub trait Transport {
    // Start the transports server/socket
    async fn start(&self) -> Result<(), TransportError>;

    // Stop the transports server/socket
    async fn stop(&self) -> Result<(), TransportError>;

    // Emit data to any listeners
    async fn emit(&self, data: Vec<u8>) -> Result<(), TransportError>;

    // Process incoming data
    async fn process(&self, data: Vec<u8>) -> Result<(), TransportError>;
}
