# Installation

How to build
```shell
git clone https://github.com/HagasSaan/sova.git
cd sova
cargo build --release
```

How to install
```shell
mkdir -p /etc/sova

cat << EOF > /etc/sova/sova.yaml
behaviour_on_incidents: KillProcess
logfile_path: "/var/log/sova.log"
rules:
EOF

echo <fullpath to sova>/fakeldap/release/libsova.so > /etc/ld.so.preload
```

Before adding your rules please read [configuration](./configuration.md) chapter.
