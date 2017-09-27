build:
	cargo build

run:
	ROCKET_PORT=8001 RUST_BACKTRACE=1 ./target/debug/shiftrss-web

run-cmd:
	./target/debug/shiftrss --file tests/data/bitemyapp_rss_small.xml --match Python

build-watch:
	cargo watch -x build

build-watch-no-warn:
	RUSTFLAGS="-A warnings" cargo watch -x build

test:
	cargo test -- --nocapture

test-debug:
	RUST_BACKTRACE=1 RUST_LOG=shiftrss=debug cargo test -- --nocapture

test-debug-watch:
	RUST_BACKTRACE=1 RUST_LOG=shiftrss=debug cargo watch -x test

fmt:
	cargo fmt

rustfix:
	rustfix
