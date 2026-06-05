all: test check_all

check_all: lint fmt_check doc unused_dep typos

test:
	cargo test

bench:
	cargo bench

fmt:
	cargo fmt

fmt_check:
	cargo fmt -- --check

fix:
	cargo fix --allow-staged

doc:
	RUSTDOCFLAGS="-D warnings" cargo doc --document-private-items --all --no-deps

check_missing_doc:
	# Warn about missing doc for public API
	RUSTDOCFLAGS="-W missing_docs" cargo doc --all --no-deps

lint:
	cargo clippy --no-deps --workspace --all-targets                -- -D warnings
	# Bug: clippy --all-targets reports false warning about unused dep in
	# `[dev-dependencies]`:
	# https://github.com/rust-lang/rust/issues/72686#issuecomment-635539688
	# Thus we only check unused deps for lib
	RUSTFLAGS=-Wunused-crate-dependencies cargo clippy --no-deps  --lib -- -D warnings

unused_dep:
	cargo machete

typos:
	# cargo install typos-cli
	typos

fix_typos:
	typos --write-changes

clean:
	cargo clean

.PHONY: all check_all test bench fmt fmt_check fix doc check_missing_doc lint unused_dep typos fix_typos clean
