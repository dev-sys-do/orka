
# Orka - Cluster Scheduler

## Requirements

### Service provided

The Scheduler is responsible for scheduling workloads across the cluster nodes.
It is the interface between the Controller and the Agent.

### Scope

The Scheduler is responsible for:

- Scheduling and re-scheduling `orka` workloads
- Nodes and Workload Instances status updates
    - Received from the Agents
    - Sent to the Controller
- Handling of Agent connection
    - The Scheduler is responsible for accepting new connections
    - Secure connection must be made between Agent and Scheduler

### Out of Scope

- Workload management (creation/update/deletion)
- Node management (connection/disconnection)
    - The Scheduler is only receiving status updates
    - The Agent is responsible for connecting to and disconnecting from the Scheduler
- Rescheduling workloads automatically in case of an Agent disconnection
    - Workload lifecycle management is the Controller's responsibility


## High Level Architecture

```mermaid
---
title: Workload Instance Scheduling
---
sequenceDiagram
    autonumber
    participant c as Controller
    participant s as Scheduler
    participant a as Agent

    c->>s: Instance scheduling request

    activate s

    s->>s: self-check of available nodes
    s->>s: determine scheduling priority

    loop for each node in priority order
        s->>a: Instance scheduling request
        
        alt Agent accepted scheduling
            a-->>s: OK
        else
            a-->>s: NOK
        end
    end

    alt one Agent accepted instance scheduling
        s-->>c: scheduling sucessful
    else
        s-->>c: Instance scheduling failed
    end

    deactivate s

```

```mermaid
---
title: Agent lifecycle
---
sequenceDiagram
    autonumber
    participant a as Agent
    participant s as Scheduler
    participant c as Controller

    activate a
    a->>a: certificate gathering

    Note left of a: Connection

    a->>+s: Connection request with certificate
    
    s->>s: Certificate verification

    alt Invalid certificate
        s-->>a: Connection denied
    else
        s-->>-a: Connection successful
    end
    deactivate a

    Note left of a: Periodic updates

    activate a
    loop periodically
        a--)+s: Node and instances update
        s--)c: Transmission of received status
    end
    deactivate a

    Note left of a: Graceful termination

    activate a
    activate s
    a--)s: disconnection notification
    s->>+c: transmission of given notification
    c->>c: rescheduling (see "Workload Instance scheduling")
    c-->>-s: OK
    deactivate s
    deactivate a

    Note left of a: Loss of contact

    activate a
    activate s
    break connection timeout
        a--)+s: timeout
        s->>+c: node timeout
        c->>c: rescheduling (see "Workload Instance scheduling")
        c-->>-s: OK
    end
    deactivate s
    deactivate a

```


## API

The Scheduler can be interacted with using gRPC. It exposes two APIs: the Controller side, and the Agent side.

Each API is defined in its gRPC `.proto` file:

- [`controller.proto`](./controller.proto)
- [`agent.proto`](./agent.proto)

