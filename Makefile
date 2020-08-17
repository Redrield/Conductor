
.PHONY: main

main:
	cd ui/main-window \
		&& npm run build:react
	RUST_LOG=info cargo run

all:
	cd ui/main-window \
		&& npm run build:react
	cd ui/stdout \
	    && npm run build
	RUST_LOG=info cargo run

setup:
	cd ui/main-window \
        && npm install
	cd ui/stdout \
        && npm install

release:
	cd ui/main-window \
	    && npm run build:react
	cd ui/stdout \
	    && npm run build
	cargo build --release
