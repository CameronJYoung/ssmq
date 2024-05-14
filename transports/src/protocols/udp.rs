use tokio::net::UdpSocket;
use async_trait::async_trait;
use crate::common::error::TransportError;

use crate::common::transport::Transport;

pub struct UdpTransport {
    address: String,
    socket: Option<UdpSocket>,
}

impl UdpTransport {
    pub fn new(address: String) -> Self {
        Self { address, socket: None }
    }
}

#[async_trait]
impl Transport for UdpTransport {
    async fn start(&self) -> Result<(), TransportError> {
        todo!()
    }

    async fn stop(&self) -> Result<(), TransportError> {
        todo!()
    }

    async fn emit(&self, data: Vec<u8>) -> Result<(), TransportError> {
        todo!()
    }

    async fn process(&self, data: Vec<u8>) -> Result<(), TransportError> {
        todo!()
    }
}