# SOVA
##### Tool to control syscalls in your Docker container
##### [Documentation](https://hagassaan.github.io/sova/)

SOVA is docker-based utility to control behaviour by capturing syscalls in the middle and reject them if they are not passing rules for current configuration.

SOVA currently supports capturing of these syscalls
- [execv](https://man7.org/linux/man-pages/man3/exec.3.html)
- [execve](https://man7.org/linux/man-pages/man2/execve.2.html)
- [open](https://man7.org/linux/man-pages/man2/open.2.html)
- [bind](https://man7.org/linux/man-pages/man2/bind.2.html)
- [connect](https://man7.org/linux/man-pages/man2/connect.2.html)

## Configuration file example

```yaml
---
behaviour_on_incidents: KillProcess
logfile_path: "/var/log/sova.log"
rules:
  execve:
  - subject: Argv
    behaviour_on_violation: KillProcess
    condition: MustNotBeIn
    objects:
      - /etc/passwd
```


## Installation

Write Dockerfile as follows

```dockerfile
FROM rust:1.66.0-bullseye

RUN mkdir /sova
WORKDIR /sova
COPY ./src /sova/src
COPY ./Cargo.toml /sova/Cargo.toml
RUN cargo build --release


### Your app installation steps

# Enable Sova
RUN echo '/sova/target/release/libsova.so' > /etc/ld.so.preload
# Add your configuration file 
COPY ./sova.yaml /etc/sova/sova.yaml
```