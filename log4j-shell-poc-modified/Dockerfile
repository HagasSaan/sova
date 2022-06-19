FROM tomcat:8.0.36-jre8

RUN rm -rf /usr/local/tomcat/webapps/*
ADD target/log4shell-1.0-SNAPSHOT.war /usr/local/tomcat/webapps/ROOT.war
EXPOSE 8080 
CMD ["catalina.sh", "run"]


RUN apt-get update || apt-get install -y build-essential 

RUN curl https://sh.rustup.rs > install.sh && chmod +x install.sh && sh install.sh -y


RUN mkdir /sova
WORKDIR /sova
COPY ./src /sova/src
COPY ./Cargo.toml /sova/Cargo.toml
RUN ~/.cargo/bin/cargo build --release

RUN touch /tmp/sova.log && chmod 777 /tmp/sova.log
COPY ./sova.build.yaml /etc/sova/sova.yaml
RUN echo '/sova/target/release/libsova.so' > /etc/ld.so.preload
COPY ./sova_envs.sh /etc/profile.d/sova_envs.sh

COPY ./sova.yaml /etc/sova/sova.yaml
