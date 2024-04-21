use std::sync::Arc;
use daemonize::Daemonize;
use std::fs::File;

use crate::interface::cli_server::CliServer;

mod interface;

#[tokio::main]
async fn main() {
    let stdout = File::create("/tmp/ssmqd.out").unwrap();
    let stderr = File::create("/tmp/ssmqd.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/ssmqd.pid")
        .chown_pid_file(true)
        .working_directory("/tmp")
        .user("nobody")
        .group("daemon")
        .stdout(stdout)
        .stderr(stderr);

    match daemonize.start() {
        Ok(_) => println!("Daemon started successfully"),
        Err(e) => eprintln!("Error starting daemon: {}", e),
    }

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
