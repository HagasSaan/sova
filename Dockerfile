FROM rust:1.58.1-bullseye

RUN mkdir /sova
WORKDIR /sova
COPY ./src /sova/src
COPY ./Cargo.toml /sova/Cargo.toml
RUN cargo build --release

COPY ./sova.build.yaml /sova/sova.build.yaml
RUN echo '/sova/target/release/libsova.so' > /etc/ld.so.preload
COPY ./sova_envs.sh /etc/profile.d/sova_envs.sh

# ----------------------------------------------------------
# Sample app
RUN apt update && apt install -y python3 python3-pip
RUN mkdir /sample_app
COPY sample_app /sample_app
RUN pip3 install -r /sample_app/requirements.txt

CMD ["python3", "/sample_app/main.py"]
# -----------------------------------------------------------

