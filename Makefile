test:
	cargo test -- --nocapture --test-threads=1

clippy:
	cargo clippy --tests

.PHONY: run
run:
	cargo run -p taylor-bin testfiles/intder.in $(ARGS)

TARGET = target/x86_64-unknown-linux-gnu/release/taylor-bin

build:
    # see https://msfjarvis.dev/posts/building-static-rust-binaries-for-linux
	RUSTFLAGS='-C target-feature=+crt-static' \
	cargo build -p taylor-bin --release --target x86_64-unknown-linux-gnu

ELAND_DEST = 'eland:bin/taylor'
WOODS_DEST = 'woods:bin/taylor'

eland: build
	scp -C ${TARGET} ${ELAND_DEST}

woods: build
	scp -C ${TARGET} ${WOODS_DEST}

install: build
	sudo ln -sf $(realpath ${TARGET}) /usr/bin/taylor
