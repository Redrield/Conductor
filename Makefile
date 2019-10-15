
.PHONY: all

all:
	cd ui \
	 && elm make src/*.elm --output=index.js \
	 && mv index.js ../web
	cargo run
