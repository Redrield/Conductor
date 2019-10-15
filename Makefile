
.PHONY: all

all:
	cd ui \
	 && elm make src/*.elm --output=index.js \
	 && mv index.js ../web
	cargo run

release:
	cd ui \
		&& elm make src/*.elm --optimize --output=index.js \
		&& mv index.js ../web
	cargo build --release
