.PHONY: all check clean

%all: ;

%check:
	cargo fmt --check
	cargo check
	cargo clippy --all-targets --all-features -- -A 'clippy::style' \
		-A 'clippy::pedantic' -A 'clippy::restriction' -D warnings

%clean:
	cargo clean
