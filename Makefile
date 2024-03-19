testw::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo watch -x test

test::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo test --verbose

fmt::
	./cargo fmt

lint::
	./cargo clippy -- -D warnings

a01::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker build-push-deploy --build-arg BACKEND_TAG=0.0.4 --config tests/01_trivial-backend-no-storage/opday.toml

b01::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker build --build-arg BACKEND_TAG=0.0.4 --config tests/01_trivial-backend-no-storage/opday.toml

p01::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker push --build-arg BACKEND_TAG=0.0.4 --config tests/01_trivial-backend-no-storage/opday.toml

d01::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker deploy --build-arg BACKEND_TAG=0.0.4 --config tests/01_trivial-backend-no-storage/opday.toml

b02::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker build --build-arg BACKEND_TAG=0.0.4 --config tests/02_simple-backend-with-database/opday.toml

p02::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker push --build-arg BACKEND_TAG=0.0.4 --config tests/02_simple-backend-with-database/opday.toml

d02::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker deploy --build-arg BACKEND_TAG=0.0.4 --config tests/02_simple-backend-with-database/opday.toml
