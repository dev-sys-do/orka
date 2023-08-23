## Bridge plugin

### Overview

The network configuration specifies the name of the bridge to be used. If the bridge is missing, the plugin will create one on first use and, if gateway mode is used, assign it an IP that was returned by IPAM plugin via the gateway field.

### Example

```conf
{
    "cniVersion": "0.3.1",
    "name": "mynet",
    "type": "bridge",
    "bridge": "mynet0",
    "isDefaultGateway": true,
    "forceAddress": false,
    "ipMasq": true,
    "hairpinMode": true,
    "ipam": {
        "type": "host-local",
        "subnet": "10.10.0.0/16"
    }
}
```

### Network configuration reference 

- `name` (string, required): the name of the network.
- `type` (string, required): “bridge”.
- `bridge` (string, optional): name of the bridge to use/create. Defaults to “cni0”.
- `isGateway` (boolean, optional): assign an IP address to the bridge. Defaults to false.
- `isDefaultGateway` (boolean, optional): Sets isGateway to true and makes the assigned IP the default route. Defaults to false.
- `forceAddress` (boolean, optional): Indicates if a new IP address should be set if the previous value has been changed. Defaults to false.
- `ipMasq` (boolean, optional): set up IP Masquerade on the host for traffic originating from this network and destined outside of it. Defaults to false.
- `mtu` (integer, optional): explicitly set MTU to the specified value. Defaults to the value chosen by the kernel.
- `hairpinMode` (boolean, optional): set hairpin mode for interfaces on the bridge. Defaults to false.
- `ipam` (dictionary, required): IPAM configuration to be used for this network. For L2-only network, create empty dictionary.
- `promiscMode` (boolean, optional): set promiscuous mode on the bridge. Defaults to false.
- `vlan` (int, optional): assign VLAN tag. Defaults to none.
- `preserveDefaultVlan` (boolean, optional): indicates whether the default vlan must be preserved on the veth end connected to the bridge. Defaults to true.
- `vlanTrunk` (list, optional): assign VLAN trunk tag. Defaults to none.
- `enabledad` (boolean, optional): enables duplicate address detection for the container side veth. Defaults to false.
- `macspoofchk` (boolean, optional): Enables mac spoof check, limiting the traffic originating from the container to the mac address of the interface. Defaults to false.

Note: The VLAN parameter configures the VLAN tag on the host end of the veth and also enables the vlan_filtering feature on the bridge interface.

Note: To configure uplink for L2 network you need to allow the vlan on the uplink interface by using the following command bridge vlan add vid VLAN_ID dev DEV.

### Example L2-only configuration

```conf
{
    "cniVersion": "0.3.1",
    "name": "mynet",
    "type": "bridge",
    "bridge": "mynet0",
    "ipam": {}
}
```

### Interface configuration arguments reference

The following `CNI_ARGS` are supported:

- `MAC`: request a specific MAC address for the interface
    
  (example: CNI_ARGS=“MAC=c2:11:22:33:44:55”)

## Execution Protocol 

### Parameters

Protocol parameters are passed to the plugins via OS environment variables.

- `CNI_COMMAND`: indicates the desired operation; ADD, DEL, CHECK, or VERSION.
- `CNI_CONTAINERID`: Container ID. A unique plaintext identifier for a container, allocated by the runtime. Must not be empty. Must start with an alphanumeric character, optionally followed by any combination of one or more alphanumeric characters, underscore (_), dot (.) or hyphen (-).
- `CNI_NETNS`: A reference to the container’s “isolation domain”. If using network namespaces, then a path to the network namespace (e.g. /run/netns/[nsname])
- `CNI_IFNAME`: Name of the interface to create inside the container; if the plugin is unable to use this interface name it must return an error.
- `CNI_ARGS`: Extra arguments passed in by the user at invocation time. Alphanumeric key-value pairs separated by semicolons; for example, “FOO=BAR;ABC=123”
- `CNI_PATH`: List of paths to search for CNI plugin executables. Paths are separated by an OS-specific list separator; for example ‘:’ on Linux and ‘;’ on Windows

### CNI operations

CNI defines 4 operations: `ADD`, `DEL`, `CHECK`, and `VERSION`. These are passed to the plugin via the `CNI_COMMAND` environment variable.

### Getting started

```bash
cat bridge.test.conf |  NERID=1111 CNI_IFNAME=eth69 CNI_NETNS=/run/netns/red ./bridge
```