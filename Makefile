
.PHONY: all

all:
	cd ui \
	 && elm make src/Main.elm --output=index.js \
	 && elm make src/StdoutWindow.elm --output=stdout.js \
	 && mv *.js ../web
	cargo run

release:
	cd ui \
	    && elm make src/lib/*.elm src/Main.elm --output=index.js --optimize \
	    && elm make src/lib/*.elm src/StdoutWindow.elm --output=stdout.js --optimize \
		&& mv *.js ../web
	cargo build --release
