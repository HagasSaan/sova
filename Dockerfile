FROM rust:1.66.1-bullseye

RUN --mount=type=cache,target=/var/cache/apt apt update && apt install -y netcat


RUN mkdir /sova
WORKDIR /sova
COPY ./src /sova/src
COPY ./Cargo.toml /sova/Cargo.toml
RUN --mount=type=cache,target=/sova/build \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build # --release

CMD ["sleep", "infinity"]

#RUN echo '/sova/target/release/libsova.so' > /etc/ld.so.preload
RUN echo '/sova/target/debug/libsova.so' > /etc/ld.so.preload
#COPY ./sova.yaml /etc/sova/sova.yaml



