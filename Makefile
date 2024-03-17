testw::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo watch -x test

test::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo test

run::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run

b1::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --config tests/dkrdeliver.test.toml build --build-arg BACKEND_TAG=0.0.3

d1::
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --config tests/dkrdeliver.test.toml deploy --build-arg BACKEND_TAG=0.0.3
