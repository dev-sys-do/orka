#!/bin/bash

# Build plugin binaries
export RUSTFLAGS='-A warnings'
mkdir -p builds/
cd ./plugins/bridge || exit
plugins_names=("bridge" "host-local" "orka-cni")
for str in "${plugins_names[@]}"; do
	cd ../"$str" || exit
  cargo build --release
  cp ./target/release/"$str" ../../builds
done

# tar them into an archive
cd ../../builds/ || exit
tar czfv ./cni_plugins.tar.gz bridge host-local orka-cni
