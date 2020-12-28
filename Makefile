.PHONY: build

build:
	cd runtime &&npm install && npm run build
	$(MAKE) -C examples/demo build
	$(MAKE) -C examples/actor build

.PHONY: clean

clean:
	cargo clean && rm -f Cargo.lock
	$(MAKE) -C examples/demo clean
	$(MAKE) -C examples/actor clean

.PHONY: test

test:
	$(MAKE) -C examples/demo test
	$(MAKE) -C examples/actor test
