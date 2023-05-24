MAKEFLAGS += --silent

build:
	mkdir -p ./app/libs/
	cd lib1 && \
	cargo build --release && \
	cp target/release/liblib1.so ../app/libs/
	cd app && \
	cargo build --release && \
	./target/release/app
