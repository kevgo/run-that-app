# Run-that-app development guide

The `Makefile` contains all development tasks. Run `make` to see a list of them
including descriptions of what they do.

### Set up your development machine

- install the latest stable Rust toolchain through [Rustup](https://rustup.rs)
- run `make setup`

### Add a new applications

Apps are in the [src/applications](../src/applications) folder. Copy the
definition of an existing application that is close to the one you want to add
and adjust the data and installation methods. You can test your installation
methods by the end-to-end test for your app (see below).

### End-to-end tests

The end-to-end tests verify that all installation methods of all apps work with
the latest version of their app. They run via an undocumented command of the RTA
executable.

Run all end-to-end tests:

```zsh
cargo run --release -- --test [--verbose]
```

Start the end-to-end test suite at a particular application:

```zsh
cargo run --release -- --test [--verbose] <app-name>
```

Running an end-to-end while actively developing the RTA executable:

```zsh
cargo run -- --test [--verbose] <app name>
```

The end-to-end test tests each install operation in a new Yard in a temporary
location. The folder is deleted when the test finishes. Re-running the tests
re-downloads all apps again.

The `--verbose` switch outputs more details around individual activities.

### Debugging

To debug the `rta` executable:

- adjust the debug parameters in .vscode/launch.json
- VSCode: Run > Start Debugging
