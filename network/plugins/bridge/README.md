# Bridge

## Getting started

```bash
docker compose up --build -d
docker exec -it bridge-brdige-1 /bin/bash
cat $NETCONFPATH/10-mynet.conf | $CNI_PATH/bridge
```