use std::fs::File;
use daemonize::Daemonize;

pub fn setup_daemon() {
    println!("Starting SSMQ daemon...");

    let log_file = File::create("/var/log/ssmq/ssmq.log").unwrap();
    let err_log_file = log_file.try_clone().expect("Unable to clone log file for stderr");

    let daemonize = Daemonize::new()
        .pid_file("/var/run/ssmq/ssmq.pid")
        .chown_pid_file(true)
        .working_directory("/var/lib/ssmq")
        .user("ssmquser")
        .group("daemon")
        .stdout(log_file)
        .stderr(err_log_file);

    match daemonize.start() {
        Ok(_) => println!("Daemon started successfully"),
        Err(e) => eprintln!("Error starting daemon: {}", e),
    }
}