AID := ./target/debug/aid

current: attach

build:
	cargo build

attach: build
	$(AID) attach cachelib-khang

run: build
	$(AID) annihilate cachelib2
	$(AID) run cachelib-khang

install:
	cargo install --all-features --path . --locked

.PHONY: attach
