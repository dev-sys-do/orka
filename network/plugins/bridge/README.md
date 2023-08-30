**Note**: This document has moved to [https://github.com/lapsus-ord/orka/blob/cni-impl/network/Documentation/bridge.md](https://github.com/lapsus-ord/orka/blob/cni-impl/network/Documentation/bridge.md).

## Debug with `cnitool`

First, install cnitool:

```bash
go get github.com/containernetworking/cni
go install github.com/containernetworking/cni/cnitool
```

Download `host-local` plugin:

```bash
make download_vendors
```

Create a network namespace. This will be called `testing`:

```bash
sudo ip netns add testing
```

**Add** the container to the network:

```bash
make add
```

**Check** whether the container's networking is as expected (ONLY for spec v0.4.0+):

```bash
make check
```

And clean up:

```bash
make del
```