FROM gradle:7.3.1-jdk17-alpine AS builder
COPY --chown=gradle:gradle . /home/gradle/src
WORKDIR /home/gradle/src
RUN gradle bootJar --no-daemon


FROM openjdk:8u181-jdk
EXPOSE 8080
RUN mkdir /app
COPY --from=builder /home/gradle/src/build/libs/*.jar /app/spring-boot-application.jar
CMD ["java", "-jar", "/app/spring-boot-application.jar"]

RUN apt-get update && apt-get install -y build-essential
RUN curl https://sh.rustup.rs > install.sh && chmod +x install.sh && sh install.sh -y

RUN mkdir /sova
WORKDIR /sova
COPY ./sova/src /sova/src
COPY ./sova/Cargo.toml /sova/Cargo.toml
RUN ~/.cargo/bin/cargo build # --release

RUN echo '/sova/target/debug/libsova.so' > /etc/ld.so.preload
