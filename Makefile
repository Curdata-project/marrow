.PHONY: build

build:
	cd runtime && npm run build
	$(MAKE) -C examples/demo build

.PHONY: clean

clean:
	$(MAKE) -C examples/demo clean

.PHONY: test

test:
	$(MAKE) -C examples/demo test
