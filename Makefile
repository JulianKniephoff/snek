.DEFAULT_GOAL := build
.PHONY: build start lint clean

RELEASE ?= 0

ifeq ($(RELEASE),1)
WASM_RELEASE_FLAG := --release
endif

lint:
	cargo clippy --target wasm32-unknown-unknown -- -D warnings

build: lint
	mkdir -p dist
	wasm-pack build --target web --out-dir dist/pkg $(WASM_RELEASE_FLAG)
	cp src/index.html dist/index.html

start: build
	python3 -m http.server 8000 --directory dist

clean:
	rm -rf dist
	cargo clean
