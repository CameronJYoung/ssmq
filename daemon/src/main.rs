use std::sync::Arc;

use crate::interface::cli_server::CliServer;

mod interface;

#[tokio::main]
async fn main() {
    println!("Starting SSMQ daemon...");

    println!("Starting CLI server...");
    let cli_server = Arc::new(CliServer::new("127.0.0.1".to_string(), 8080));
    let cli_server_handle = {
        let cli_server = Arc::clone(&cli_server);
        tokio::spawn(async move {
            if let Err(e) = cli_server.start().await {
                eprintln!("Failed to start CLI server: {}", e);
            }
        })
    };

    cli_server_handle.await.unwrap();
}
