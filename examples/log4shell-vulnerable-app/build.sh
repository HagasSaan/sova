mkdir -p sova
cp -r ../../src sova
cp ../../Cargo.toml sova
docker-compose up -d --build
rm -rf sova