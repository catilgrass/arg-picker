.PHONY: doc doc-preview build-picker build-test-crate build test-picker test-macros test-test-crate test clippy-picker clippy-macros clippy-test-crate clippy check package release

VERSION ?=

doc:
	cargo doc --no-deps --features=derive

doc-preview:
	python3 -m http.server 3000

build-picker:
	cargo build --manifest-path Cargo.toml --features derive

build-test-crate:
	cargo build --manifest-path test/Cargo.toml

build: build-picker build-test-crate

test-picker:
	cargo test --manifest-path Cargo.toml --features derive

test-macros:
	cargo test --manifest-path macros/Cargo.toml --features derive

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

package:
	mkdir -p target
	echo '[workspace]' > target/Cargo.toml
	cargo package --workspace --allow-dirty

release: package
	@if [ -z "$(VERSION)" ]; then \
		echo "Error: VERSION is required. Usage: make release VERSION=x.y.z"; \
		exit 1; \
	fi

	rm -f target/package/arg-picker-$(VERSION)/Cargo.toml.orig
	rm -f target/package/arg-picker-macros-$(VERSION)/Cargo.toml.orig

	rm -f target/package/arg-picker-$(VERSION)/Cargo.lock
	rm -f target/package/arg-picker-macros-$(VERSION)/Cargo.lock

	cd target/package/arg-picker-macros-$(VERSION) && cargo build --features=derive
	cd target/package/arg-picker-macros-$(VERSION) && cargo test --features=derive
	cd target/package/arg-picker-macros-$(VERSION) && cargo clippy -- -D warnings

	cd target/package/arg-picker-macros-$(VERSION) && cargo publish --dry-run
	cd target/package/arg-picker-macros-$(VERSION) && cargo publish

	cd target/package/arg-picker-$(VERSION) && cargo build --features=derive
	cd target/package/arg-picker-$(VERSION) && cargo test --features=derive
	cd target/package/arg-picker-$(VERSION) && cargo clippy -- -D warnings

	cd target/package/arg-picker-$(VERSION) && cargo publish --dry-run
	cd target/package/arg-picker-$(VERSION) && cargo publish

check: build test clippy
