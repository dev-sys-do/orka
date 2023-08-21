# Orka SDN

## Getting started (dev)

To test our CNI plugin, we will use [`cnitool`](https://github.com/containernetworking/cni/tree/main/cnitool),
a tool in go to execute CNI configuration.

> Install it easily with: `make cnitool-install`

To test the plugin, you can execute:

```sh
make cni-setup
make cni-add # or any other CNI method you want to use
```

To clean the created resources:

```sh
make cni-clean
```
