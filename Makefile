
.PHONY: all

all:
	cd ui \
	 && elm make src/lib/*.elm src/Main.elm --output=index.js \
	 && elm make src/lib/*.elm src/StdoutWindow.elm --output=stdout.js \
	 && mv *.js ../web
	cargo run

release:
	cd ui \
		&& elm make src/*.elm --optimize --output=index.js \
		&& mv index.js ../web
	cargo build --release
