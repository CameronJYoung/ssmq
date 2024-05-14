use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Notify;

pub struct CliServer {
    pub address: String,
    pub port: u16,
    should_stop: Arc<AtomicBool>,
    notify_stop: Arc<Notify>,
    notify_stopped: Arc<Notify>,
}

impl CliServer {
    pub fn new(address: String, port: u16) -> Self {
        Self {
            address,
            port,
            should_stop: Arc::new(AtomicBool::new(false)),
            notify_stop: Arc::new(Notify::new()),
            notify_stopped: Arc::new(Notify::new()),
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        println!("Starting CLI Server on {}:{}", self.address, self.port);
        let listener = TcpListener::bind(format!("{}:{}", self.address, self.port)).await?;
        println!("CLI Server running on {}:{}", self.address, self.port);

        while !self.should_stop.load(Ordering::SeqCst) {
            println!("Waiting for a new connection...");
            tokio::select! {
            Ok((stream, _)) = listener.accept() => {
                println!("Hello");
                let stop_clone = self.should_stop.clone();
                let stopped_clone = self.notify_stopped.clone();
                tokio::spawn(async move {
                    if let Err(e) = Self::handle_connection(stream, stop_clone).await {
                        eprintln!("CLI Connection closed: {}", e);
                    }
                    stopped_clone.notify_one(); // Notify that one connection is closed
                });
            },
            _ = self.notify_stop.notified() => {
                break; // Break out of the loop when notified to stop
            },
        }
        }

        println!("CLI server is shutting down");
        self.notify_stopped.notify_waiters();
        Ok(())
    }

    pub async fn stop(&self) {
        self.should_stop.store(true, Ordering::SeqCst);
        self.notify_stop.notify_waiters();
        self.notify_stopped.notified().await;
    }

    async fn handle_connection(mut stream: TcpStream, should_stop: Arc<AtomicBool>) -> Result<(), Box<dyn Error>> {
        let mut buf = vec![0; 1024];

        println!("connection established");

        while !should_stop.load(Ordering::SeqCst) {
            match stream.read(&mut buf).await {
                Ok(n) if n == 0 => return Err("Connection closed by client".into()),
                Ok(n) => {
                    let received = String::from_utf8_lossy(&buf[..n]);
                    println!("Received: {}", received.trim());
                    match received.trim() {
                        "status" => {
                            let response = "Daemon is running\n";
                            stream.write_all(response.as_bytes()).await?;
                        },
                        _ => {
                            let response = "Unknown command\n";
                            stream.write_all(response.as_bytes()).await?;
                        },
                    }
                },
                Err(e) => return Err(e.into()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

    #[tokio::test]
    async fn test_cli_server_start_stop() {
        let cli_server = Arc::new(CliServer::new("127.0.0.1".to_string(), 8081));

        let server_handle = {
            let cli_server = Arc::clone(&cli_server);
            tokio::spawn(async move {
                if let Err(e) = cli_server.start().await {
                    eprintln!("Failed to start CLI server: {}", e);
                }
            })
        };

        tokio::time::sleep(Duration::from_millis(500)).await;

        assert!(TcpStream::connect("127.0.0.1:8081").await.is_ok());

        cli_server.stop().await;

        let _ = server_handle.await;

        assert!(TcpStream::connect("127.0.0.1:8081").await.is_err()); // Expect the server to be unreachable
    }

    #[tokio::test]
    async fn test_cli_server_status_response() {
        let cli_server = Arc::new(CliServer::new("127.0.0.1".to_string(), 8082));

        let server_handle = {
            let cli_server = Arc::clone(&cli_server);
            tokio::spawn(async move {
                if let Err(e) = cli_server.start().await {
                    eprintln!("Failed to start CLI server: {}", e);
                }
            })
        };

        tokio::time::sleep(Duration::from_millis(500)).await;

        let mut tcp_client = TcpStream::connect("127.0.0.1:8082").await.unwrap();

        let mut buf = vec![0; 1024];
        tcp_client.write_all(b"status\n").await.unwrap();
        let n = tcp_client.read(&mut buf).await.unwrap();
        let response = String::from_utf8_lossy(&buf[..n]);

        assert_eq!(response, "Daemon is running\n");

        cli_server.stop().await;

        let _ = server_handle.await;
    }

    #[tokio::test]
    async fn test_cli_server_unknown_command() {
        let cli_server = Arc::new(CliServer::new("127.0.0.1".to_string(), 8083));

        let server_handle = {
            let cli_server = Arc::clone(&cli_server);
            tokio::spawn(async move {
                if let Err(e) = cli_server.start().await {
                    eprintln!("Failed to start CLI server: {}", e);
                }
            })
        };

        tokio::time::sleep(Duration::from_millis(500)).await;

        let mut tcp_client = TcpStream::connect("127.0.0.1:8083").await.unwrap();

        let mut buf = vec![0; 1024];
        tcp_client.write_all(b"unknown\n").await.unwrap();
        let n = tcp_client.read(&mut buf).await.unwrap();
        let response = String::from_utf8_lossy(&buf[..n]);

        assert_eq!(response, "Unknown command\n");

        cli_server.stop().await;

        let _ = server_handle.await;
    }
}
