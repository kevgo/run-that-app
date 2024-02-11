build:  # compiles this app in debug mode
	cargo build

fix: build  # auto-corrects issues
	cargo fix --allow-dirty
	cargo clippy --fix
	cargo fmt
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

test: unit lint  # runs all tests

unit:  # runs the unit tests
	cargo test

update:  # updates the dependencies
	cargo upgrade


.DEFAULT_GOAL := help
.SILENT:
