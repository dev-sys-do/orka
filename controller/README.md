# Controller gRPC implementing task scheduling

## Description

This project is a gRPC implementation (server and client) that allows you to manage the state of a Kubernetes cluster. The gRPC server (controller) receives information about the current state of the cluster from the scheduler, saves this new state in a database, and retrieves the requested state from the database (workload). The controller then uses the scheduling function to schedule changes if the state is not good.

## Installation

>**_Warning_**: When installing protocol buffers compiler ([protoc](https://grpc.io/docs/protoc-installation/)), make sure to install at least the version 3.15.x . Watch out by default `apt` installs version 3.12.x. To install the latest version required you can follow the steps bellow.

1. Curl the latest release of protoc from protoc github :
```
$ PB_REL="https://github.com/protocolbuffers/protobuf/releases"
$ curl -LO $PB_REL/download/v3.15.8/protoc-3.15.8-linux-x86_64.zip
```

2. Unzip the file under $HOME/.local or a directory of your choice :

```
$ unzip protoc-3.15.8-linux-x86_64.zip -d $HOME/.local
```
3. Update your environment’s path variable to include the path to the protoc executable :

```
$ export PATH="$PATH:$HOME/.local/bin"
```

## Usage

### Run the server and client GRPC

1. Run the server GRPC :
```
cargo run --bin server
```

2. Run the client GRPC :
```
cargo run --bin client
```