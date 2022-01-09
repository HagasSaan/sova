FROM rust:1.57.0-bullseye

RUN apt update && apt install -y gcc gzip make procps socat tar wget curl
RUN wget -O install-snoopy.sh https://github.com/a2o/snoopy/raw/install/install/install-snoopy.sh && \
    chmod 755 install-snoopy.sh
RUN bash install-snoopy.sh 2.4.15
COPY snoopy.ini.template /etc/snoopy.ini

RUN mkdir /sova
WORKDIR /sova
COPY ./src /sova/src
COPY ./Cargo.toml /sova/Cargo.toml
RUN cargo build --release

RUN echo 'sleep infinity' >> /bootstrap.sh
RUN chmod +x /bootstrap.sh
CMD /bootstrap.sh

#CMD cargo run
