FROM rust:1.70 AS builder
WORKDIR /
COPY . .
RUN cargo install --path .


FROM ubuntu:22.04 AS runner
WORKDIR /
RUN apt-get update && apt-get upgrade -y && apt-get install -y iproute2 wget
RUN mkdir -p /opt/cni/bin
RUN mkdir -p /opt/cni/netconfs
COPY --from=builder /usr/local/cargo/bin/bridge /opt/cni/bin
COPY ./test/bridge.conf /opt/cni/netconfs
COPY ./test/run.sh /opt/run.sh
RUN wget https://github.com/containernetworking/plugins/releases/download/v1.3.0/cni-plugins-linux-amd64-v1.3.0.tgz
RUN tar -xvf cni-plugins-linux-amd64-v1.3.0.tgz -C /opt/cni/bin ./host-local
RUN chmod +x /opt/run.sh
CMD [ "/opt/run.sh" ]