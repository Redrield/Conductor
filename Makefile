
.PHONY: all

all:
	cd ui-react \
		&& yarn run build:react
	RUST_LOG=info cargo run

release:
	cd ui \
	    && elm make src/Main.elm --output=index.js --optimize \
	    && elm make src/StdoutWindow.elm --output=stdout.js --optimize \
		&& mv *.js ../web
	cargo build --release
