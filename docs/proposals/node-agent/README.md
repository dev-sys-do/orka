# Node Agent

## Requirements

### Scope

The node agent manages `orka` workloads on a specific node. There is one agent per node.

* A node is a physical or virtual machine in the cluster, that is managed by the `orka` control plane.
* A workload is the smallest unit of compute deployable in orka. It can be e.g. a container, a virtual machine, etc.

The Node Agent provides an API  for controlling workloads lifecycle on the node it manages, and a node monitoring interface:

- to control the workloads :
  - start
  - stop
  - restart
  - kill
- health of node
- metrics of node
- status of workloads
  - waiting
  - running
  - terminated
- keeping workloads running/alive
- cleaning up terminated workloads

#### Out of scope

- Networking configuration.
- Workload and node status storage. This is the responsibility of the `orka` controller.
- Workload rescheduling.

## High level architecture

### Interfaces

See the [node agent gRPC specification](docs/proposals/node-agent/agent.proto)
