CLIPPY_FLAGS = --all-targets --all-features -- -A 'clippy::style' \
               -A 'clippy::pedantic' -A 'clippy::restriction' \
               -A 'clippy::cargo' -D warnings

.PHONY: all check clean

%all: ;

%check:
	cargo fmt --check
	cargo check
	cargo clippy $(CLIPPY_FLAGS)

%clean:
	cargo clean
