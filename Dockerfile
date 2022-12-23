FROM rust:1.66.0-bullseye

RUN mkdir /sova
WORKDIR /sova
COPY ./src /sova/src
COPY ./Cargo.toml /sova/Cargo.toml
RUN cargo build # --release

# ----------------------------------------------------------
# Sample app
RUN apt update && apt install -y python3 python3-pip
RUN mkdir /sample_app
COPY sample_app /sample_app
RUN pip3 install -r /sample_app/requirements.txt

CMD ["python3", "/sample_app/main.py"]
# ----------------------------------------------------------

#RUN echo '/sova/target/release/libsova.so' > /etc/ld.so.preload
RUN echo '/sova/target/debug/libsova.so' > /etc/ld.so.preload
COPY ./sova.yaml /etc/sova/sova.yaml


