use clap::Command;

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
            println!("Checking SSMQ daemon status...");
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
                .about("Starts the SSMQ daemon")
        )
        .subcommand(
            Command::new("stop")
                .about("Stops the SSMQ daemon")
        )
        .subcommand(
            Command::new("status")
                .about("Returns the status of the SSMQ daemon")
        )
}
