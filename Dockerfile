FROM rust:1.58.1-bullseye

RUN mkdir /sova
WORKDIR /sova
COPY ./src /sova/src
COPY ./Cargo.toml /sova/Cargo.toml
RUN cargo build --release

ENV LD_PRELOAD=/sova/target/release/libsova.so

CMD ["sleep", "infinity"]
