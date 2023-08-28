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
  - [Section 2: Interface configuration arguments reference](#section-2-interface-configuration-arguments-reference)


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

## Section 2: Interface configuration arguments reference 

The following `CNI_ARGS` are supported:


| Implemented | Field          | Description                                                                                   |
| ----------- | -------------- | --------------------------------------------------------------------------------------------- |
| ❌           | `MAC` (string) | Request a specific MAC address for the interface (example: CNI_ARGS=“MAC=c2:11:22:33:44:55”). |
