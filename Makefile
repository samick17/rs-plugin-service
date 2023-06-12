MAKEFLAGS += --silent

setup-wasm:
	rustup target add wasm32-unknown-unknown

build-wasm:
	mkdir -p ./app/libs/
	cd libs/lib1 && \
	cargo build --target wasm32-unknown-unknown --release && \
	cp target/wasm32-unknown-unknown/release/lib1.wasm ../../app/libs/

build-so:
	mkdir -p ./app/libs/
	cd libs/lib1 && \
	cargo build --release && \
	cp target/release/liblib1.so ../../app/libs/

build:
	cd app && \
	cargo build --release && \
	./target/release/app

broker:
	cd app && ROLE=broker cargo run

producer:
	cd app && ROLE=producer cargo run

consumer:
	cd app && ROLE=consumer cargo run
