FROM rust:1.58.1-bullseye

RUN mkdir /sova
WORKDIR /sova
COPY ./src /sova/src
COPY ./Cargo.toml /sova/Cargo.toml
RUN cargo build --release

COPY ./sova.build.yaml /etc/sova/sova.yaml
ENV SOVA_CONFIG=/etc/sova/sova.yaml
RUN echo '/sova/target/release/libsova.so' > /etc/ld.so.preload

# ----------------------------------------------------------
# Sample app
RUN apt update && apt install -y python3 python3-pip
RUN mkdir /sample_app
COPY sample_app /sample_app
RUN pip3 install -r /sample_app/requirements.txt

CMD ["python3", "/sample_app/main.py"]

COPY ./sova.yaml /etc/sova/sova.yaml

# -----------------------------------------------------------

