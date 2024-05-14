use std::sync::Arc;
use tokio::io;

use crate::interface::cli_server::CliServer;
use crate::system::setup_daemon::setup_daemon;

mod interface;
mod system;

fn main() {
    setup_daemon();

    tokio_main();
}

#[tokio::main]
async fn tokio_main() {
    println!("Starting CLI server...");
    let cli_server = Arc::new(CliServer::new("127.0.0.1".to_string(), 8080));

    if let Err(e) = cli_server.start().await {
        eprintln!("Error starting CLI server: {}", e);
    }

    println!("CLI server stopped");
}
