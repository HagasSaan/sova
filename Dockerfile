FROM rust:1.66.1-bullseye

RUN apt update && apt install -y netcat


RUN mkdir /sova
WORKDIR /sova
COPY ./src /sova/src
COPY ./Cargo.toml /sova/Cargo.toml
RUN cargo build # --release

CMD ["sleep", "infinity"]

#RUN echo '/sova/fakeldap/release/libsova.so' > /etc/ld.so.preload
RUN echo '/sova/fakeldap/debug/libsova.so' > /etc/ld.so.preload
#COPY ./sova.yaml /etc/sova/sova.yaml



