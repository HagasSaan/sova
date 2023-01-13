mkdir -p target/sova
cp -r ../../src target/sova
cp ../../Cargo.toml target/sova
docker-compose up -d --build
rm -rf target/sova
