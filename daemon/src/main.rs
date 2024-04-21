use std::sync::Arc;
use daemonize::Daemonize;
use std::fs::File;

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
            if let Err(e) = cli_server.start().await {
                eprintln!("Failed to start CLI server: {}", e);
            }
        })
    };

    cli_server_handle.await.unwrap();
}
