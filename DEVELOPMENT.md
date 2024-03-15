# Run-that-app development guide

The `Makefile` contains all development tasks. Run `make` to see a list of them
including descriptions of what they do.

### Add a new applications

Apps are in the [src/apps](src/apps) folder. Copy the definition of an existing
application that is close to the one you want to add and adjust the data and
installation methods. You can test your installation methods by the end-to-end
test for your app (see below).

### End-to-end tests

The end-to-end tests verify that all installation methods of all apps work with
the latest version of their app. They run via an undocumented command of the RTA
executable.

````fish
rta --test [--verbose] [app name]
```

or while developing on app definitions:

```fish
cargo run -- --test [--verbose] [app name]
````

The end-to-end test creates a Yard in a temporary location and deletes the
installation when done. Re-running the tests re-downloads all apps again.

The `--verbose` switch outputs more details around individual activities.

If you provide an application name, the end-to-end test only tests this
application, otherwise tests all apps.
