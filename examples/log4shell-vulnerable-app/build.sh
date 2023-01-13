mkdir -p app/sova
cp -r ../../src app/sova
cp ../../Cargo.toml app/sova
docker-compose up -d --build
rm -rf app/sova
