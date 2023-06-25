# Software Defined Network

## Analysis

### Examples

- [Flannel](https://www.tkng.io/cni/flannel/)
- [Kindnet](https://www.tkng.io/cni/kindnet/)

### Sources

- [CNI](https://github.com/containernetworking/cni) also make a good framework for creating a new container networking project from scratch
- [Networks - treeptik k8s](https://treeptik.gitbook.io/k8s/fundamentals/)
- [Explain NS linux](https://www.youtube.com/watch?v=j_UUnlVC2Ss)
- [Spec - CNI](https://www.cni.dev/docs/spec/)
- [TKNG - The Kubernetes Networking Guide](https://www.tkng.io/)

## Network domain

### Glossary

- *runtime* is the program responsible for executing CNI plugins.
- *plugin* is a program that applies a specified network configuration.  
  (it's a binary or an executable)

### Definition

The **orka SDN** implements the [`CNI`](https://www.cni.dev/docs/spec/) specification.

It is a **CNI plugin** and must be called by e.g. the node Agent through the `CNI` specified
command line interface. It configure workloads networking according to a set of `CNI`
compliant configuration files, as described in the following sections.

The goal is to be able to:

- manage networks through the software
- isolate instance/pods with isolated networks
- apply network security policies

The *plugins* must be installed on every node in the cluster.

### Scope

- Pod-to-Pod network packets on the same host
	- Firewall (e.g. iptables)
		- Need to ACCEPT FORWARDING traffic for pod CIDR because packets are dropped by the Linux kernel by default  
		  (Linux treats network packets in non-default network namespaces as external packets by default)
	- Allow outgoing internet
- Cross-node pod-to-pod network packets
- Port mapping / Selectors pods
- DNS routing (e.g. ingress controller)
- DNS resolution (e.g. coreDNS) -> pod-to-pod and pod-to-service
	- "<NAME_SVC>.<NAMESPACE>.svc.cluster.local"
	- "<POD_IP_WITH_DASH_INSTEAD_DOT>.<NAMESPACE>.pod.cluster.local"

### Out of scope

- RBAC, Network Policies
- Monitoring network traffic (e.g. metrics-server)
- Load balancing (e.g. service)
- Load balancer (e.g. klipper, metallb)

## API - Component interface

### CNI

#### Provision phase

At the node creation, the node agent needs to download/set the binaries and the configuration file:

- the CNI plugin is located in `/opt/cni/bin/`
- [configuration files](https://www.cni.dev/docs/spec/#example-configuration) (`.conf`) are located in `/etc/cni/net.d/`

Our configuration file for the orka CNI plugin (`10-orka-cni.conf`):

```json
{
  "cniVersion": "1.0.0",
  "name": "orknet",
  "plugins": [
    {
      "type": "orka-cni",
      "subnet": "10.1.0.0/16"
    }
  ]
}
```

#### Runtime phase

There are 4 methods available:

- `ADD` create a network interface
- `DELETE` delete a network interface
- `CHECK` check if the configuration is as expected
- `VERSION` the CNI version of the plugin

The runtime passes parameters to the plugin via:

- [protocol parameters](https://www.cni.dev/docs/spec/#parameters) via environment variables (is invocation-specific)
- [configuration](https://www.cni.dev/docs/spec/#example-configuration) via stdin (is the same for any given network)

The plugin returns a result on stdout on success, or an error on stderr if the operation fails.
Configuration and results are encoded in JSON.

> For more, see here: ["Appendix: Examples – cni.dev"](https://www.cni.dev/docs/spec/#appendix-examples)
