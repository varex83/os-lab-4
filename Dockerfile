FROM rust:latest as builder

RUN USER=root cargo new --bin os-lab-4
WORKDIR /os-lab-4

COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

FROM ubuntu:latest
RUN apt-get update && apt-get install -y \
    procps screen \
    && rm -rf /var/lib/apt/lists/*


COPY --from=builder /os-lab-4/target/release/secret /usr/local/bin/secret
COPY --from=builder /os-lab-4/target/release/mem /usr/local/bin/mem
COPY --from=builder /os-lab-4/target/release/hook /usr/local/bin/hook

#PID: 12
#Address of marker: 0xffffe311c494
