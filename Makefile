build:  # compiles this app in debug mode
	cargo build

fix: build  # auto-corrects issues
	cargo fix
	cargo clippy --fix
	cargo fmt
	target/debug/run-that-app dprint fmt
	target/debug/run-that-app shfmt -f . | xargs target/debug/run-that-app shfmt -w

install:  # installs this tool locally for testing
	cargo install --path .

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT:' | grep '#' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

lint: build  # runs all linters
	cargo clippy --all-targets --all-features -- --deny=warnings
	git diff --check
	target/debug/run-that-app actionlint
	# target/debug/run-that-app dprint check  # this breaks the Windows CI
	# target/debug/run-that-app/shfmt -f . | xargs target/debug/run-that-app/shellcheck

test: unit lint  # runs all tests

unit:  # runs the unit tests
	cargo test

update:  # updates the dependencies
	cargo upgrade


.DEFAULT_GOAL := help
.SILENT:
