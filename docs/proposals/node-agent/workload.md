# Workloads

The node agent manages workload management. A workload is the tiniest unit in orka. It can be a container, a virtual machine, etc.

At the moment, `orka` only supports container workloads.

## Workload definition

```json
{
  "id": "d9ed9738-c400-464d-9e5a-d950ba470200",
  "name": "my_workload",
  "type": "container",
  "image": "nginx",
  "status": "RUNNING",
  "environment": ["FEEDING_TIME=midnight"],
  "resources": {
    "cpu": 1.0,
    "memory": "256m"
  }
}
```
