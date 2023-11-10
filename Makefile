DPRINT_VERSION = 0.42.5
SHELLCHECK_VERSION = 0.9.0
SHFMT_VERSION = 3.7.0

.DEFAULT_GOAL := help
.SILENT:

build:  # compiles this app in debug mode
	cargo build

fix: build  # auto-corrects issues
	cargo fix
	cargo fmt
	target/debug/run-that-app dprint@${DPRINT_VERSION} fmt
	target/debug/run-that-app/shfmt@${SHFMT_VERSION} -f . | xargs "target/debug/run-that-app/shfmt@${SHFMT_VERSION}" -w

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT:' | grep '#' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

lint: build  # runs all linters
	cargo clippy --all-targets --all-features -- -Dwarnings -W clippy::pedantic -A clippy::module-inception
	git diff --check
	# target/debug/run-that-app/shfmt@${SHFMT_VERSION} -f . | xargs target/debug/run-that-app/shellcheck@${SHELLCHECK_VERSION}

test: unit lint  # runs all tests

unit:  # runs the unit tests
	cargo test

update:  # updates the dependencies
	cargo upgrade
