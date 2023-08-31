# Software Defined Network: `orkanet`

## Overview

The main plugin used by the runtime (or the CRI) is `orka-cni`,
this plugin will then delegate the creation of interfaces and IPAM
to other plugins like `bridge` and `host-local`.

The inspiration for this plugin comes from the [CNI plugin](https://github.com/flannel-io/cni-plugin)
of flannel.

## Summary

- [Software Defined Network: `orkanet`](#software-defined-network-orkanet)
  - [Overview](#overview)
  - [Summary](#summary)
  - [Section 1: Protocol parameters](#section-1-protocol-parameters)
    - [Environment variables](#environment-variables)
    - [Errors](#errors)
    - [CNI operations](#cni-operations)
  - [Section 2: Getting started](#section-2-getting-started)


## Section 1: Protocol parameters

Protocol parameters are passed to the plugins via OS environment variables.

### Environment variables

- `CNI_COMMAND`: indicates the desired operation; ADD, DEL, CHECK or VERSION.
- `CNI_CONTAINERID`: Container ID. A unique plaintext identifier for a container, allocated by the runtime. Must not be empty. Must start with an alphanumeric character, optionally followed by any combination of one or more alphanumeric characters, underscore (_), dot (.) or hyphen (-)
- `CNI_NETNS`: A reference to the container's “isolation domain”. If using network namespaces, then a path to the network namespace (e.g., `/run/netns/[nsname]`)
- `CNI_IFNAME`: Name of the interface to create inside the container; if the plugin is unable to use this interface name it must return an error
- `CNI_ARGS`: Extra arguments passed in by the user at invocation time. Alphanumeric key-value pairs separated by semicolons; for example, “FOO=BAR;ABC=123”
- `CNI_PATH`: List of paths to search for CNI plugin executables. Paths are separated by an OS-specific list separator; for example ‘:’ on Linux and ‘;’ on Windows

### Errors

A plugin must exit with a return code of 0 on success, and non-zero on failure. If the plugin encounters an error, it should output an “error” result structure (see below).

### CNI operations

CNI defines 4 operations: `ADD`, `DEL`, `CHECK`, and `VERSION`. These are passed to the plugin via the `CNI_COMMAND` environment variable.

## Section 2: Getting started

To test our CNI plugin, you can use [`cnitool`](https://github.com/containernetworking/cni/tree/main/cnitool),
it is a tool in go to execute CNI configuration.
