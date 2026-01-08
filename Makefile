CLICOLOR_FORCE = 1
export CLICOLOR_FORCE

build:  # compiles this app in debug mode
	cargo build --locked

contest: build
	target/debug/rta contest

doc: build node_modules  # test the documentation
	target/debug/rta npm exec text-runner

fix: build  # auto-corrects issues
	cargo +nightly fix --allow-dirty
	cargo clippy --fix --allow-dirty
	cargo +nightly fmt
	target/debug/rta dprint fmt
	target/debug/rta shfmt -f . | xargs target/debug/rta shfmt -w
	target/debug/rta keep-sorted $(shell CLICOLOR_FORCE=1 target/debug/rta ripgrep -l 'keep-sorted end' ./ --glob '!Makefile')

install:  # installs this tool locally for testing
	cargo install --locked --path .

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT:' | grep '#' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

lint: build  # runs all linters
	cargo clippy --all-targets --all-features -- --deny=warnings
	git diff --check
	target/debug/rta actionlint
	# target/debug/rta dprint check  # this breaks the Windows CI due to linebreak errors
	target/debug/rta --optional shellcheck download.sh

setup:  # install development dependencies on this computer
	rustup component add clippy
	rustup toolchain add nightly
	rustup component add rustfmt --toolchain nightly

test: fix unit lint doc  # runs all tests

todo:  # displays all TODO items
	@git grep --color=always --line-number TODO ':!target' | grep -v Makefile

unit: build node_modules  # runs the unit tests
	cargo test --locked
	target/debug/rta node --test 'text-runner/**/*.test.ts'

update:  # updates the dependencies
	cargo install cargo-edit cargo-machete
	cargo machete
	cargo upgrade
	cargo run -- --update


# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

.DEFAULT_GOAL := help
.SILENT:

node_modules: package.json package-lock.json
	target/debug/rta npm ci
	@touch node_modules  # update timestamp so that Make doesn't re-install it on every command
