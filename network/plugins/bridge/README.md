**Note**: This document has moved to [https://github.com/lapsus-ord/orka/blob/cni-impl/network/Documentation/bridge.md](https://github.com/lapsus-ord/orka/blob/cni-impl/network/Documentation/bridge.md).

## Test plugin locally

To test the plugin locally, follow these steps:

1. Navigate to the "test" directory:

```bash
cd test
```

2. Build and run the Docker Compose environment in detached mode:

```bash
docker compose up --build -d
```

3. Enter the running container named "bridge-bridge-1" using an interactive shell:

```bash
docker exec -it bridge /bin/bash
```

4. Execute the "bridge" CNI plugin:
   
```bash
cat $NETCONFPATH/bridge.conf | $CNI_PATH/bridge
```