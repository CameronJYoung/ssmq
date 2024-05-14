use clap::Command;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand_name() {
        Some("start") => {
            println!("Starting SSMQ daemon...");
        },
        Some("stop") => {
            println!("Stopping SSMQ daemon...");
        },
        Some("status") => {
            ssmq_status_check().await;
        },
        _ => {
            println!("Invalid command. Use 'ssmq-cli --help' for more information.");
        }
    }
}

fn cli() -> Command {
    Command::new("ssmq-cli")
        .about("A CLI used to interact with SSMQ (Super Simple Message Queue)")
        .override_usage("ssmq-cli <COMMAND>")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("start")
                .about("Starts the SSMQ daemon (WIP)")
        )
        .subcommand(
            Command::new("stop")
                .about("Stops the SSMQ daemon (WIP)")
        )
        .subcommand(
            Command::new("status")
                .about("Returns the status of the SSMQ daemon")
        )
}

async fn ssmq_status_check() {
    let mut tcp_client = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    let mut buf = vec![0; 1024];
    tcp_client.write_all(b"status\n").await.unwrap();
    let n = tcp_client.read(&mut buf).await.unwrap();
    let response = String::from_utf8_lossy(&buf[..n]);

    println!("{}", response);
}
