ORKA_NETWORK_NS ::= "orka-$(shell uuidgen)" # will change only at each `make` command
CNI_PATH = "${PWD}/target/debug/orka-cni"
NETCONFPATH = "${PWD}/examples"

### CNI UTILS
cni-setup:        ## Setup resources to execute CNI plugins
	@sudo ip netns add $(ORKA_NETWORK_NS)
	@echo "Create a network namespace at /var/run/netns/$(ORKA_NETWORK_NS)"


cni-clean:        ## Clean all created resources
	@for ns in $$(ip netns list | awk '/^orka-/{print $$1}'); do \
    	sudo ip netns delete $$ns; \
    done

cnitool-install:  ## Install `cnitool` binary
	go install github.com/containernetworking/cni/cnitool@latest

##
### CNI METHODS
cni-add:          ## Run cnitool with ADD method
	cargo build
#	CNI_PATH=$(CNI_PATH) NETCONFPATH=$(NETCONFPATH) cnitool add orknet /var/run/netns/$(ORKA_NETWORK_NS)

# TODO write other CNI commands

##
### MISC
help:             ## Show this help
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'
