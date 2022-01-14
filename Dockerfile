FROM rust:1.58.0-bullseye

RUN apt update && apt install -y gcc gzip make procps socat tar wget curl supervisor
RUN wget -O install-snoopy.sh https://github.com/a2o/snoopy/raw/install/install/install-snoopy.sh && \
    chmod 755 install-snoopy.sh
RUN bash install-snoopy.sh 2.4.15
COPY conf/snoopy.ini /etc/snoopy.ini

RUN mkdir /sova
WORKDIR /sova
COPY ./src /sova/src
COPY ./Cargo.toml /sova/Cargo.toml
RUN cargo build --release

COPY conf/supervisord.conf /etc/supervisor/conf.d/supervisord.conf
COPY conf/sova.conf /etc/supervisor/conf.d/sova.conf
CMD ["/usr/bin/supervisord"]

# ----------------------------------------------------------
# Sample app
RUN apt update && apt install -y python3 python3-pip
RUN mkdir /sample_app
COPY sample_app /sample_app
RUN pip3 install -r /sample_app/requirements.txt
RUN chmod +x /sample_app/main.py

COPY sample_app/sample_app.conf /etc/supervisor/conf.d/sample_app.conf
# -----------------------------------------------------------
