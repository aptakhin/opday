test::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo test

run::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run

b1::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- build --build-arg BACKEND_TAG=0.0.3

d1::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- deploy --build-arg BACKEND_TAG=0.0.3
