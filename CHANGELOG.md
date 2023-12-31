# run-that-app changelog

### 0.3.0 (2023-12-18)

#### BREAKING CHANGES

- renames the executable from `run-that-app` to `rta`
- renames `--show-path` to `--which`

#### New Features

- prints the name of the app being installed
- apps: Node.js, npm, deadcode

### 0.2.1 (2023-12-05)

#### Bug Fixes

- updates to the release marked as latest on GitHub

### 0.2.0 (2023-12-05)

#### New Features

- `run-that-app --update` updates the versions in `.tool-versions`

### 0.1.1 (2023-12-02)

- apps: ghokin

### 0.1.0 (2023-11-30)

#### New Features

- `--available` command indicates via the exit code whether an application is
  available
- `--show-path` command displays the path of the executable instead of running
  it
- `--optional` parameter makes the app do nothing if the app isn't available
- `--include-global` parameter runs a globally installed app if the app cannot
  be installed
- config file (.tool-versions) for defining the versions of tools to run

#### Bug Fixes

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
- apps: alphavet, depth, dprint, gh, gofumpt, golangci-lint, scc, shellcheck,
  shfmt
- logging with namespaces for downloading and extracting
- shell output
