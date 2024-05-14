mkdir /var/lib/ssmq # This will create the directory for the daemon

mkdir /var/log/ssmq # This will create the log directory for the daemon

cargo build --release # This will build the application binaries (daemon, cli)

sudo useradd -r -s /bin/false ssmquser # Create the ssmquser user

sudo chown -R ssmquser:ssmquser /var/lib/ssmq # Change ownership of the directory to the ssmquser user

cp target/release/daemon /usr/local/sbin/ssmq-daemon # This will copy the daemon to the sbin directory

chmod +x /usr/local/sbin/ssmq-daemon # This will make the daemon executable

sudo setcap 'cap_net_bind_service=+ep' /usr/local/sbin/ssmq-daemon # Give the user the appropriate perms

/usr/local/sbin/ssmq-daemon # This will start the daemon

cp target/release/cli /usr/local/bin/ssmq-cli # This will copy the cli to the bin directory

chmod +x /usr/local/bin/ssmq-cli # This will make the cli executable
