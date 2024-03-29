testw::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo watch -x test

test::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo test --verbose

fmt::
	./cargo fmt

fmt-check::
	./cargo fmt --check

lint::
	./cargo clippy -- -D warnings

doc::
	./cargo doc --no-deps

docw::
	./cargo watch -x doc --no-deps

l01::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker login --config tests/01_trivial-backend-no-storage/opday.toml -f ./secrets/docker-config.json

a01::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker build-push-deploy --build-arg BACKEND_TAG=0.0.4 --config tests/01_trivial-backend-no-storage/opday.toml

b01::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker build --build-arg BACKEND_TAG=0.0.4 --config tests/01_trivial-backend-no-storage/opday.toml

p01::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker push --build-arg BACKEND_TAG=0.0.4 --config tests/01_trivial-backend-no-storage/opday.toml

d01::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker deploy --build-arg BACKEND_TAG=0.0.4 --config tests/01_trivial-backend-no-storage/opday.toml

b02::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker build --build-arg BACKEND_TAG=0.0.4 --build-arg NGINX_TAG=0.0.1 --config tests/02_simple-backend-with-database/opday.toml

p02::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker push --build-arg BACKEND_TAG=0.0.4 --build-arg NGINX_TAG=0.0.1 --config tests/02_simple-backend-with-database/opday.toml

d02::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker deploy --build-arg BACKEND_TAG=0.0.4 --build-arg NGINX_TAG=0.0.1 --config tests/02_simple-backend-with-database/opday.toml

c02::
	curl http://46.101.98.131/api/v1/make-database-call
