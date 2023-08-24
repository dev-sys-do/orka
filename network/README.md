# Orka SDN

The main plugin used by the runtime (or the CRI) is `orka-cni`,
this plugin will then delegate the creation of interfaces and IPAM
to other plugins like `bridge` and `host-local`.

The inspiration for this plugin comes from the [cni plugin](https://github.com/flannel-io/cni-plugin)
of flannel.

## Getting started

To test our CNI plugin, you can use [`cnitool`](https://github.com/containernetworking/cni/tree/main/cnitool),
it is a tool in go to execute CNI configuration.
