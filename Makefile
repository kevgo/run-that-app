RTA          = target/debug/rta
ACTIONLINT   = $(RTA) actionlint
DPRINT       = $(RTA) dprint
KEEP_SORTED  = $(RTA) keep-sorted
NPM          = $(RTA) npm
NODE         = $(RTA) node
RIPGREP      = $(RTA) ripgrep
RUMDL        = $(RTA) rumdl
SHELLCHECK   = $(RTA) --optional shellcheck
TAPLO        = $(RTA) taplo
TEXTRUNNER   = $(NPM) exec text-runner

build:  # compiles this app in debug mode
	cargo build --locked

contest: build
	target/debug/rta contest

doc: build node_modules  # test the documentation
	$(TEXTRUNNER)

fix: build  # auto-corrects issues
	cargo +nightly fix --allow-dirty
	cargo clippy --fix --allow-dirty
	cargo +nightly fmt
	$(DPRINT) fmt
	$(RUMDL) fmt
	$(TAPLO) fmt
	CLICOLOR_FORCE=1 target/debug/rta shfmt -f . | xargs target/debug/rta shfmt -w
	$(KEEP_SORTED) $(shell $(RIPGREP) -l 'keep-sorted end' ./ --glob '!Makefile')

install:  # installs this tool locally for testing
	cargo install --locked --path .

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT:' | grep '#' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

lint: build  # runs all linters
	cargo clippy --all-targets --all-features -- --deny=warnings
	git diff --check
	$(ACTIONLINT)
	# $(DPRINT) check
	$(RUMDL) check
	$(TAPLO) check
	$(SHELLCHECK) download.sh

ps: fix test  # pitstop

setup:  # install development dependencies on this computer
	rustup component add clippy
	rustup toolchain add nightly
	rustup component add rustfmt --toolchain nightly

test: unit lint doc  # runs all tests

todo:  # displays all TODO items
	@git grep --color=always --line-number TODO ':!target' | grep -v Makefile

unit: build node_modules  # runs the unit tests
	cargo test --locked --quiet
	$(NODE) --test 'text-runner/**/*.test.ts'

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
