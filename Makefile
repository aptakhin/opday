testw::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo watch -x test

test::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo test

run::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run

b1::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --config tests/dkrdeliver.test.toml docker build --build-arg BACKEND_TAG=0.0.4

p1::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --config tests/dkrdeliver.test.toml docker push --build-arg BACKEND_TAG=0.0.4

d1::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --config tests/dkrdeliver.test.toml docker deploy --build-arg BACKEND_TAG=0.0.4
