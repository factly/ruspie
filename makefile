.PHONY: build-web
build-web:
	npm start --prefix web


.PHONY: build-server
build-server:
	cargo run --manifest-path ./server/Cargo.toml

.PHONY: build
build: build-web && build-server

