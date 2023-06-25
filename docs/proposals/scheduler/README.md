
# Orka - Cluster Scheduler

## Requirements

### Service provided

The Scheduler is responsible for the scheduling of nodes and workloads.
It is the interface between the Controller and the Agent.

### Scope

The Scheduler is reponsible for:

- Sending workload scheduling/re-scheduling
    - Re-scheduling can happen upon Agent disconnection
- Nodes and Workload Instances status updates
    - received from the Agents
    - sent to the Controller
- Handling of Agent connection
    - the Scheduler is responsible for the handshake
    - secure connection should be made between Agent and Scheduler

The Scheduler is NOT responsible for:

- Workload management (creation/update/deletion)
- Node management (connection/disconnection)
    - the Scheduler is only receiving status updates
    - the Agent is responsible for its connection and disconnection to the Scheduler
- Rescheduling workloads automatically in case of an Agent disconnection
    - this is the Controller's responsbility
    - the Controller needs to call the Scheduler with a reschedule order


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

The Scheduler is interactable through gRPC. It exposes two APIs: the Controller side, and the Agent side.

Each API is defined in its gRPC `.proto` file:

- [`controller.proto`](./controller.proto)
- [`agent.proto`](./agent.proto)

