# Controller gRPC implementing task scheduling

## Description

This is the `orka` controller reference implementation. The `orka` controller manages the overall state of an `orka` cluster through one public `gRPC` API and an internal one. The former allows for clients to control an `orka` cluster and e.g. create new workload instances, while the latter is consumed by the scheduler component to synchronously send node and instances status updates.

This project provides both a public API sample implementation and the internal API reference implementation.
## Installation

### Protobuf

>**_Warning_**: The protocol buffers compiler ([protoc](https://grpc.io/docs/protoc-installation/)) version 3.15.x or later is required to build this project. Follow the steps below to install it manually if your distribution installs an earlier version of the `protoc` toolchain.

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
## Logging

The `orka` controller uses the [pretty_env_logger](https://docs.rs/pretty_env_logger/latest/pretty_env_logger/) crate to log all helpful information. The log level can be set by setting the `RUST_LOG` environment variable. For example, to set the log level to `trace` you can run the following command :

```
export RUST_LOG=info
```


## Usage

### Run the server and client GRPC

1. Run the server the controller :
```
cargo run --bin orka-controller
```

