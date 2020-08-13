
.PHONY: all

setup:
	cd ui \
        && npm install

all:
	cd ui \
		&& npm run build:react
	RUST_LOG=info cargo run

release:
	cd ui \
	    && npm run build:react
	cargo build --release
