# run-that-app changelog

### 0.19.1 (2025-10-03)

- fix wrong message when config file not found

### 0.19.0 (2025-09-22)

- app: keep-sorted

### 0.18.0 (2025-06-16)

- app: funcorder

### 0.17.0 (2025-06-05)

- renamed config file from .app-versions to .run-that-app

### 0.16.0 (2025-06-04)

- renamed config file from .tools-versions to .app-versions

### 0.15.1 (2025-05-22)

- reduced binary size

### 0.15.0 (2025-04-19)

- adds `--from-source` flag to always install an app from source

### 0.14.2 (2025-04-13)

- no longer break config file format while updating versions

### 0.14.1 (2025-03-28)

- improves compatibility with older libc versions

### 0.14.0 (2025-02-28)

- "add" command

### 0.13.0 (2025-02-17)

- can now call applications by application name: `rg@14.1.1` instead of `ripgrep@14.1.1`

### 0.12.0 (2025-02-15)

- added ripgrep

### 0.11.0 (2025-02-03)

- `--include` option to include other files into the PATH

### 0.10.6 (2025-01-27)

- extracting archives makes executables properly executable

### 0.10.5 (2025-01-27)

- npm and npm: run correctly on first run

### 0.10.4 (2025-01-24)

- npm and npx: run correctly on Windows
- depth: fix Windows download
- go: fix tags
- staticcheck: fix installation from source
- node-prune: fix installation from source
- ireturn: fix installation from source
- govulncheck: fix identification
- goreleaser: fix identification, no longer installs from source
- ghokin: fix archive download
- gh: fix executable path on Windows
- node: fix executable path on Windown
- prints "not found" messages in yellow instead of red now because they are not necessarily error conditions

### 0.10.3 (2025-01-13)

- fix the filepath of the Windows archive for mdbook

### 0.10.2 (2025-01-13)

- makes the executable file of an application executable if needed after extracting from archive

### 0.10.1 (2025-01-13)

- fixes the path to the mdbook-linkcheck executable when installing from source

### 0.10.0 (2025-01-13

- apps: mdbook-linkcheck

### 0.9.0 (2024-12-28)

- Can now compile Go-based tools using an RTA-installed Go toolchain ([#237](https://github.com/kevgo/run-that-app/issues/237)).
- Adds executables of the application to run to the PATH of the subshell ([#298](https://github.com/kevgo/run-that-app/issues/298)).

### 0.8.1 (2024-10-19)

- apps: node-prune

### 0.8.0 (2024-10-19)

- list of apps is now displayed via the `--apps` (for a complete list) or `-a` (for the app names only) switch
- apps: tikibase

### 0.7.1 (2024-09-15)

- finds the mdbook executable in a subfolder after installation from source

### 0.7.0 (2024-09-15)

- apps: govulncheck, staticcheck

### 0.6.1 (2024-05-31)

- fix bug when compiling ghokin from source
- improve CLI output format
- massively more robust code without possibilities for crashes and panics

### 0.6.0 (2024-05-05)

- apps: exhaustruct, ireturn

### 0.5.0 (2024-03-14)

The `.tool-versions` file can now define multiple versions. RTA tries versions from left to right until it finds one that it can run on your hardware.

The `--which` command now returns a non-zero exit code if the given app isn't available on your machine.

All apps now have all-lowercase names. The `mdBook` app is now `mdbook`.

When running externally installed apps, _run-that-app_ now verifies that the executable it found is actually is the app. It also
determines whether the version of the globally installed application matches version restrictions declared by your code base.

End-to-end tests: run `cargo run -- --test` to verify that all installation methods of all apps work for the latest app version. See `DEVELOPMENT.md` for details.

### 0.4.1 (2024-02-29)

- fixed installation of `scc` from source

### 0.4.0 (2024-02-11)

- can now execute in subfolders of the folder that contains the `.tools-versions` file
- `--error-on-output` option
- print available versions using `--versions` and `--versions=<amount>`
- apps: go, goda, gofmt, npx, mdBook

### 0.3.0 (2023-12-18)

- renames the executable from `run-that-app` to `rta`
- renames `--show-path` to `--which`
- prints the name of the app being installed
- apps: Node.js, npm, deadcode

### 0.2.1 (2023-12-05)

- updates to the release marked as latest on GitHub

### 0.2.0 (2023-12-05)

- `run-that-app --update` updates the versions in `.tool-versions`

### 0.1.1 (2023-12-02)

- apps: ghokin

### 0.1.0 (2023-11-30)

- `--available` command indicates via the exit code whether an application is available
- `--show-path` command displays the path of the executable instead of running it
- `--optional` parameter makes the app do nothing if the app isn't available
- `--include-global` parameter runs a globally installed app if the app cannot be installed
- config file (.tool-versions) for defining the versions of tools to run
- bugfix: install from Go source
- bugfix: install alphavet from source

### 0.0.5 (2023-11-28)

- apps: actionlint

### 0.0.4 (2023-11-23)

- installation of run-that-app on PowerShell
- bugfix: gh on Windows

### 0.0.3 (2023-11-20)

- apps: add goreleaser

### 0.0.2 (2023-11-17)

- compression: add `.tar.xz` support
- apps: fixes for golangci-lint and shellcheck

### 0.0.1 (2023-11-14)

- installation methods: downloading binaries, compile from source
- apps: alphavet, depth, dprint, gh, gofumpt, golangci-lint, scc, shellcheck, shfmt
- logging with namespaces for downloading and extracting
- shell output
