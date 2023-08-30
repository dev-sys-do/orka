PLUGINS_PATH='./target/debug:./vendors'
NET_CONF_PATH='./test'
CNITOOL_PATH='~/go/bin/cnitool'

ORKA_CNI_BIN='./target/debug/orka-cni'
NETWORK_NAME='orknet'
NS_NAME='testing'

.cargo-build:
	@cargo build

add: .cargo-build
	@sudo CNI_PATH=$(PLUGINS_PATH) NETCONFPATH=$(NET_CONF_PATH) $(CNITOOL_PATH) add $(NETWORK_NAME) /var/run/netns/$(NS_NAME)
	@echo ''

del: .cargo-build
	@sudo CNI_PATH=$(PLUGINS_PATH) NETCONFPATH=$(NET_CONF_PATH) $(CNITOOL_PATH) del $(NETWORK_NAME) /var/run/netns/$(NS_NAME)
	@echo ''

check: .cargo-build
	@sudo CNI_PATH=$(PLUGINS_PATH) NETCONFPATH=$(NET_CONF_PATH) $(CNITOOL_PATH) check $(NETWORK_NAME) /var/run/netns/$(NS_NAME)
	@echo ''

version: .cargo-build
	@echo '{"cniVersion": "1.0.0", "supportedVersions": [ "0.1.0", "0.2.0", "0.3.0", "0.3.1", "0.4.0", "1.0.0" ]}' | sudo CNI_COMMAND=VERSION $(ORKA_CNI_BIN)
	@echo ''

download_vendors:
	@mkdir -p ./vendors
	@wget -O ./vendors.tgz https://github.com/containernetworking/plugins/releases/download/v1.3.0/cni-plugins-linux-amd64-v1.3.0.tgz
	@tar -xvf vendors.tgz -C ./vendors ./host-local
	@rm ./vendors.tgz