CARGO = cargo
NPM = npm

build_npm:
	$(NPM) install
	$(NPM) build

build_cargo:
	$(CARGO) build --target wasm32-unknown-unknown --release

build: build_npm build_cargo

clean_npm:
	rm -rf node_modules/
	rm -f package-lock.json

clean_cargo:
	$(CARGO) clean
	rm -f Cargo.lock

clean: clean_npm clean_cargo

test:
	node index.js


