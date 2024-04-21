use std::sync::Arc;

use crate::interface::cli_server::CliServer;
use crate::system::setup_daemon::setup_daemon;

mod interface;
mod system;

#[tokio::main]
async fn main() {
    setup_daemon();

    println!("Starting CLI server...");
    let cli_server = Arc::new(CliServer::new("127.0.0.1".to_string(), 8080));

    let cli_server_handle = {
        let cli_server = Arc::clone(&cli_server);
        tokio::spawn(async move {
            match cli_server.start().await {
                Ok(_) => println!("CLI server started successfully"),
                Err(e) => eprintln!("Failed to start CLI server: {}", e)
            }
        })
    };

    cli_server_handle.await.unwrap();
}
