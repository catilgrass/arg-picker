.PHONY: doc doc-preview build-picker build-test-crate build test-picker test-macros test-test-crate test clippy-picker clippy-macros clippy-test-crate clippy check

doc:
	cargo doc --no-deps

doc-preview:
	python3 -m http.server 3000

build-picker:
	cargo build --manifest-path Cargo.toml

build-test-crate:
	cargo build --manifest-path test/Cargo.toml

build: build-picker build-test-crate

test-picker:
	cargo test --manifest-path Cargo.toml

test-macros:
	cargo test --manifest-path macros/Cargo.toml

test-test-crate:
	cargo test --manifest-path test/Cargo.toml

test: test-picker test-macros test-test-crate

clippy-picker:
	cargo clippy --manifest-path Cargo.toml --all-targets --all-features -- -D warnings

clippy-macros:
	cargo clippy --manifest-path macros/Cargo.toml --all-targets --all-features -- -D warnings

clippy-test-crate:
	cargo clippy --manifest-path test/Cargo.toml --all-targets --all-features -- -D warnings

clippy: clippy-picker clippy-macros clippy-test-crate

check: build test clippy
