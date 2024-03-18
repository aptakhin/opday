testw::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo watch -x test

test::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo test

run::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run

b1::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- docker build --build-arg BACKEND_TAG=0.0.4 --config tests/01_trivial-backend-no-storage/dkrdeliver.toml

p1::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- docker push --build-arg BACKEND_TAG=0.0.4 --config tests/01_trivial-backend-no-storage/dkrdeliver.toml

d1::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- docker deploy --build-arg BACKEND_TAG=0.0.4 --config tests/01_trivial-backend-no-storage/dkrdeliver.toml

lint::
	cargo clippy
