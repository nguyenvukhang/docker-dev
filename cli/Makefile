AID := ./target/debug/aid

current: list

build:
	cargo build

list: build
	$(AID) ls

attach: build
	$(AID) attach cachelib-khang

run: build
	$(AID) annihilate cachelib2
	$(AID) run cachelib-khang

install:
	cargo install --all-features --path . --locked

.PHONY: build
