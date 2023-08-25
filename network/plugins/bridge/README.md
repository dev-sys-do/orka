# Bridge

## Getting started

```bash
docker compose up --build -d
docker exec -it bridge-brdige-1 /bin/bash
cat $NETCONFPATH/bridge.test.conf | $CNI_PATH/bridge
```