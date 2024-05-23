# Super Simple Messaging Queue (SSMQ)

THIS IS A WORK IN PROGRESS, A LOT OF THE FUNCTIONALITY DESCRIBED BELOW IS NOT YET IMPLEMENTED 

SSMQ is my attempt to create a simple messaging queue that can be used to send messages from and to different transports.
It will follow a topic based approach where messages are sent to a topic and then received by a subscriber. Currently, the
application will only run on Unix based systems. This project is no way meant to be used in production. It is a learning
project for me to understand messaging queues more deaply.

## Features

- Messaging over different transports (UDP & TCP). With the flexibility to add more transports.
- Secure topic based messaging. Only subscribers to a topic can receive messages from that topic.
- Multiple queue disciplines (FIFO, LIFO, Random). With the flexibility to add more disciplines.
- Logging (messages and metrics).

## Architecture

The architecture of SSMQ will consist of a daemon that is driven by static configuration files. The daemon will be 
responsible for managing the different transports, topics, disciplines and logging.

This repository will contain the following components:
- Daemon
- Transport Library
- Discipline Library
- Command line interface (only for starting, stopping and checking the status of the daemon)

## Transports

Transports are the different ways in which messages can be sent and received. The initial transports that will be
implemented are:

- UDP
- TCP

Transports are contained within the Transport cargo module. Each transport will have a send and receive function that
will be called by the daemon.

## Topics

Topics are the different channels that messages can be sent and received on. As a core feature of SSMQ, topics will be 
managed by the daemon. Each transport will have a way to subscribe to a topic and receive messages from that topic. Topics 
will be defined by static config initially with plans to add dynamic topic creation in future. A topic can have multiple
transports subscribed to it. Listeners will only receive messages from topics they are subscribed to for security. A topic 
can only have one discipline.

## Disciplines

Disciplines are the different ways in which messages can be queued and dequeued. The initial disciplines that will be
implemented are:

- FIFO (First In First Out)
- LIFO (Last In First Out)
- Random

Disciplines are contained within the Discipline cargo module. Each discipline will have a queue and dequeue function that
will be called by the daemon.

## Logging

Logging will be implemented within the daemon itself. The daemon will log messages and metrics to a log file determined by
the static configuration.

## Message Guarantee

For the moment the only guarantee that will be provided is that the message will be sent. There will be no guarantee that
the message will be received. In future, we will add an acknowledgement system to ensure that messages are received.

## Configuration

The daemon will be driven by a static configuration file. The configuration file will contain the following:

- daemon.toml
- topics.toml

The daemon.toml file will contain the configuration for the daemon itself. The topics.toml file will contain the configuration
for the topics.

The daemon.toml file will contain the following:

```toml
[daemon]
log_metrics = true # Log metrics to the log file
log_messages = true # Log messages to the log file
```

The topics.toml file will largely be requirement based. So as an example we have one topic called test that uses the 
UDP and TCP. It will also have a discipline of FIFO.

```toml
[topics]

# Test topic with UDP and TCP transports
[topics.test]
transports = ["UDP", "TCP"] # The transports this topic allows
discipline = "FIFO" # The discipline this topic uses (FIFO, LIFO, Random)
```

## Usage

The daemon will be started by running the following command:

```bash
$ ssmq-cli start
```

The daemon will read the configuration files and start the transports and topics. The daemon will then be ready to send
and receive messages. The daemon will log messages and metrics to the log file specified in the configuration.

The daemon can be stopped by running the following command:

```bash
$ ssmq-cli stop
```

You can check the status of the daemon by running the following command:

```bash
$ ssmq-cli status
```


## Installation

Currently, the daemon can be installed from source using the following script:

```bash
$ mkdir /var/lib/ssmq # This will create the directory for the daemon

$ cargo build --release # This will build the application binaries (daemon, cli)

$ cp target/release/daemon /usr/local/sbin/ssmq-daemon # This will copy the daemon to the sbin directory

$ chmod +x /usr/local/sbin/ssmq-daemon # This will make the daemon executable

$ /usr/local/sbin/ssmq-daemon # This will start the daemon

$ cp target/release/cli /usr/local/bin/ssmq-cli # This will copy the cli to the bin directory

$ chmod +x /usr/local/bin/ssmq-cli # This will make the cli executable

$ source ~/.bashrc # This will refresh the bash profile
```

## Uninstall

The daemon can be uninstalled using the following script:

```bash
$ kill -SIGTERM $(cat /var/run/ssmq.pid) # This will stop the daemon

$ rm /usr/local/sbin/ssmq-daemon # This will remove the daemon

$ rm -rf /var/lib/ssmq # This will remove the daemon directory

$ rm /var/run/ssmq.pid # This will remove the daemon pid file

$ rm /var/log/ssmq.log # This will remove the daemon log file

$ rm /usr/local/bin/ssmq-cli # This will remove the cli

$ source ~/.bashrc # This will refresh the bash profile
```
