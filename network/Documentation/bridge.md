# bridge plugin

## Overview

With bridge plugin, all containers (on the same host) are plugged into a bridge (virtual switch) that resides in the host network namespace. The containers receive one end of the veth pair with the other end connected to the bridge. An IP address is only assigned to one end of the veth pair – one residing in the container. The bridge itself can also be assigned an IP address, turning it into a gateway for the containers. 

The network configuration specifies the name of the bridge to be used. If the bridge is missing, the plugin will create one on first use and, if gateway mode is used, assign it an IP that was returned by IPAM plugin via the gateway field.

## Summary

- [bridge plugin](#bridge-plugin)
  - [Overview](#overview)
  - [Summary](#summary)
  - [Section 1: Network configuration reference](#section-1-network-configuration-reference)
    - [Required keys](#required-keys)
    - [Optional keys](#optional-keys)
    - [Example configuration](#example-configuration)
  - [Section 2: Protocol parameters](#section-2-protocol-parameters)
    - [Environment variables](#environment-variables)
    - [Errors](#errors)
    - [CNI operations](#cni-operations)


## Section 1: Network configuration reference

This section provides details about the configuration options for the "bridge" CNI plugin.

### Required keys

| Implemented | Field           | Description              |
| ----------- | --------------- | ------------------------ |
| ✅           | `name` (string) | The name of the network. |
| ✅           | `type` (string) | “bridge”.                |

### Optional keys

| Implemented | Field                           | Description                                                                                                                                                                |
| ----------- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ✅           | `bridge` (string)               | Name of the bridge to use/create. Defaults to “cni0”.                                                                                                                      |
| ✅           | `isGateway` (boolean)           | Assign an IP address to the bridge. Defaults to false.                                                                                                                     |
| ✅           | `isDefaultGateway` (boolean)    | Sets isGateway to true and makes the assigned IP the default route. Defaults to false.                                                                                     |
| ❌           | `forceAddress` (boolean)        | Indicates if a new IP address should be set if the previous value has been changed. Defaults to false.                                                                     |
| ❌           | `ipMasq` (boolean)              | Set up IP Masquerade on the host for traffic originating from this network and destined outside of it. Defaults to false.                                                  |
| ✅           | `mtu` (integer)                 | Explicitly set MTU to the specified value. Defaults to the value chosen by the kernel.                                                                                     |
| ❌           | `hairpinMode` (boolean)         | Set hairpin mode for interfaces on the bridge. Defaults to false.                                                                                                          |
| ✅           | `ipam` (dictionary)             | IPAM configuration to be used for this network. Refer to [host-local](https://github.com/lapsus-ord/orka/blob/cni-impl/network/Documentation/host-local.md) documentation. |
| ✅           | `promiscMode` (boolean)         | Set promiscuous mode on the bridge. Defaults to false.                                                                                                                     |
| ❌           | `vlan` (integer)                | Assign VLAN tag. Defaults to none.                                                                                                                                         |
| ❌           | `preserveDefaultVlan` (boolean) | Indicates whether the default vlan must be preserved on the veth end connected to the bridge. Defaults to true.                                                            |
| ❌           | `vlanTrunk` (list)              | Assign VLAN trunk tag. Defaults to none.                                                                                                                                   |
| ❌           | `enabledad` (boolean)           | Enables duplicate address detection for the container side veth. Defaults to false.                                                                                        |
| ❌           | `macspoofchk` (boolean)         | Enables mac spoof check, limiting the traffic originating from the container to the mac address of the interface. Defaults to false.                                       |


Note: The VLAN parameter configures the VLAN tag on the host end of the veth and also enables the vlan_filtering feature on the bridge interface.

### Example configuration

Here's an example configuration for the "bridge" CNI plugin:

```conf
{
  "cniVersion": "1.0.0",
  "name": "orknet",
  "type": "bridge",
  "bridge": "ork0",
  "isDefaultGateway": true,
  "ipam": {
    "type": "host-local",
    "subnet": "10.244.0.0/24",
  }
}
```

This example demonstrates how to configure the "bridge" plugin, specifying the network name, type, bridge name, default gateway settings, and IPAM configuration.

## Section 2: Protocol parameters 

Protocol parameters are passed to the plugins via OS environment variables.

### Environment variables

- `CNI_COMMAND`: indicates the desired operation; ADD, DEL, CHECK, or VERSION.
- `CNI_CONTAINERID`: Container ID. A unique plaintext identifier for a container, allocated by the runtime. Must not be empty. Must start with an alphanumeric character, optionally followed by any combination of one or more alphanumeric characters, underscore (_), dot (.) or hyphen (-).
- `CNI_NETNS`: A reference to the container’s “isolation domain”. If using network namespaces, then a path to the network namespace (e.g. /run/netns/[nsname])
- `CNI_IFNAME`: Name of the interface to create inside the container; if the plugin is unable to use this interface name it must return an error.
- `CNI_ARGS`: Extra arguments passed in by the user at invocation time. Alphanumeric key-value pairs separated by semicolons; for example, “FOO=BAR;ABC=123”
- `CNI_PATH`: List of paths to search for CNI plugin executables. Paths are separated by an OS-specific list separator; for example ‘:’ on Linux and ‘;’ on Windows

### Errors

A plugin must exit with a return code of 0 on success, and non-zero on failure. If the plugin encounters an error, it should output an “error” result structure (see below).

### CNI operations

CNI defines 4 operations: `ADD`, `DEL`, `CHECK`, and `VERSION`. These are passed to the plugin via the `CNI_COMMAND` environment variable.