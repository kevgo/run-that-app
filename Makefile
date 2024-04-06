build:  # compiles this app in debug mode
	cargo build

fix: build  # auto-corrects issues
	cargo +nightly fix --allow-dirty
	cargo clippy --fix --allow-dirty
	cargo +nightly fmt
	target/debug/rta dprint fmt
	target/debug/rta shfmt -f . | xargs target/debug/rta shfmt -w

install:  # installs this tool locally for testing
	cargo install --path .

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT:' | grep '#' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

lint: build  # runs all linters
	cargo clippy --all-targets --all-features -- --deny=warnings
	git diff --check
	target/debug/rta actionlint
	# target/debug/rta dprint check  # this breaks the Windows CI due to linebreak errors
	target/debug/rta --optional shellcheck download.sh

setup:  # install development dependencies on this computer
	rustup toolchain add nightly
	rustup component add rustfmt --toolchain nightly

test: unit lint  # runs all tests

unit:  # runs the unit tests
	cargo test

update:  # updates the dependencies
	cargo install cargo-machete
	cargo machete
	cargo install cargo-edit
	cargo upgrade


.DEFAULT_GOAL := help
.SILENT:
