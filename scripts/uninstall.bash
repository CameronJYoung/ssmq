# Stop the daemon
kill $(cat /var/run/ssmq/ssmq.pid)

# Remove the daemon and CLI binaries
rm -f /usr/local/sbin/ssmq-daemon
rm -f /usr/local/bin/ssmq-cli

# Remove the user
sudo userdel ssmquser

# Remove log files, PID file, and application directory
rm -rf /var/log/ssmq
rm -f /var/run/ssmq/ssmq.pid
rm -rf /var/lib/ssmq

echo "Uninstallation complete."