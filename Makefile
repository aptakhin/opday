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

build-release::
	./cargo build --release

login-01::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker login --config tests/01_trivial-backend-no-storage/opday.toml -f ./.secrets/docker-config.json

all-01::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker build-push-deploy --build-arg BACKEND_TAG=0.0.4 --config tests/01_trivial-backend-no-storage/opday.toml

build-01::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker build --build-arg BACKEND_TAG=0.0.4 --config tests/01_trivial-backend-no-storage/opday.toml

push-01::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker push --build-arg BACKEND_TAG=0.0.4 --config tests/01_trivial-backend-no-storage/opday.toml

deploy-01::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker deploy --build-arg BACKEND_TAG=0.0.4 --config tests/01_trivial-backend-no-storage/opday.toml

all-02::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker build-push-deploy --build-arg BACKEND_TAG=0.0.4 --build-arg NGINX_TAG=0.0.1 --config tests/02_simple-backend-with-database/opday.toml

build-02::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker build --build-arg BACKEND_TAG=0.0.4 --build-arg NGINX_TAG=0.0.1 --config tests/02_simple-backend-with-database/opday.toml

push-02::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker push --build-arg BACKEND_TAG=0.0.4 --build-arg NGINX_TAG=0.0.1 --config tests/02_simple-backend-with-database/opday.toml

deploy-02::
	RUST_LOG=debug RUST_BACKTRACE=1 ./cargo run -- docker deploy --build-arg BACKEND_TAG=0.0.4 --build-arg NGINX_TAG=0.0.1 --config tests/02_simple-backend-with-database/opday.toml

curl-02::
	curl http://46.101.98.131/api/v1/make-database-call
