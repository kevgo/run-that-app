<br>
<div align="center">
<img alt="Run that app logo" src="docs/logo.png" width="725" height="177" align="center">
</div>

<br><br>

_Run-that-app_ is a minimalistic cross-platform application runner. It executes
native CLI tools on Linux, macOS, Windows, and BSD without requiring a prior
installation. The primary use case is running developer tools (linters,
analyzers, formatters, etc) in scripts and CI pipelines.

#### integrating installation and execution

Installing small developer tools at pinned versions across multiple operating
systems is a surprisingly hard problem without a good solution.

Run-that-app sidesteps the problem entirely: instead of _installing_ tools, it
focuses on _running_ them. For most development workflows, that's what you
actually care about.

#### radically minimalistic

Run-that-app is intentionally minimalistic and non-invasive. It ships as a
single stand-alone binary.

Following the principle "perfection is achieved not when there is nothing left
to add, but when there is nothing left to take away", run-that-app avoids:

- environment variables
- application shims
- shell integrations
- dependencies
- plugins
- custom packaging and container formats
- a dedicated install step
- application repositories
- Docker
- WASM
- system daemons
- sudo
- emulation
- IDE plugins
- any other kind of bloat

Applications are downloaded directly their original hosting location, typically
in 1-2 seconds. Only the executable is stored on disk. Execution is 100% native,
with no runtime overhead.

[![linux](https://github.com/kevgo/run-that-app/actions/workflows/ci_linux.yml/badge.svg)](https://github.com/kevgo/run-that-app/actions/workflows/ci_linux.yml)
[![windows](https://github.com/kevgo/run-that-app/actions/workflows/ci_windows.yml/badge.svg)](https://github.com/kevgo/run-that-app/actions/workflows/ci_windows.yml)

## installation

Linux and macOS:

```sh
curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh
```

Windows (Powershell):

```powershell
Invoke-Expression (Invoke-WebRequest -Uri "https://raw.githubusercontent.com/kevgo/run-that-app/main/download.ps1" -UseBasicParsing).Content
```

The installer places the _run-that-app_ executable into the current directory.
To install elsewhere, execute the installer from that directory.

## usage

```sh
rta [run-that-app arguments] <app name>[@<app version override>] [app arguments]
```

Run [actionlint](https://github.com/rhysd/actionlint) at version `1.6.26`:

```sh
./rta actionlint@1.6.26
```

The app version should contain only the version number (e.g. `1.6.26`), even if
the Git tag is prefixed (e.g. `v1.6.26`).

_Run-that-app_ arguments must appear before the name of the application to run.
The application name is the first argument that does not start with a dash. All
following arguments are passed through to the application.

Run [ShellCheck](https://shellcheck.net) version `0.9.0` with arguments
`--color=always myscript.sh`:

```bash
rta shellcheck@0.9.0 --color=always myscript.sh
```

### list all runnable applications

```sh
rta --apps
```

### graceful degredation

Not all applications support all platforms. The `--optional` flag skips
unsupported applications without failing the command.

Run ShellCheck only if it is available on the current platform:

```bash
rta --optional shellcheck@0.9.0 myscript.sh
```

The `--available` command reports availability via its exit code.

### get the path to the installed executable

The `--which` command prints the path to the resolved executable.

Example: run `go vet` with `alphavet` as a custom vet tool, but only if
`alphavet` is available:

```sh
rta --available alphavet && go vet "-vettool=$(rta --which alphavet)" ./...
```

### monitor output

Some tools (e.g. [deadcode](https://pkg.go.dev/golang.org/x/tools/cmd/deadcode))
report findings via stdout but exit with status code 0. The `--error-on-output`
treats any output as failure.

```sh
rta --error-on-output deadcode
```

### list available versions

Show the 10 most recent versions of an application:

```sh
rta --versions actionlint
```

Limit the output to a specific number:

```sh
rta --versions=3 actionlint
```

### force installation from source

If precompiled binaries are available (e.g. via GitHub releases), _run-that-app_
use them. If not, it can compile applications from source.

You enforce compilation from source even when binaries exist:

```sh
rta --from-source <app>
```

## configuration

_Run-that-app_ supports a configuration file name `run-that-app`, using the
[asdf version file format](https://asdf-vm.com/manage/configuration.html):

```
actionlint 1.6.26 shellcheck 0.9.0
```

With this file in place, you no longer need to be specify the version
explicitly:

```bash
rta actionlint
```

The file name intentionally differs from [asdf](#asdf) and [mise](#mise) to
avoid interference.

### add an application

Add an application at its latest version (creates the config file if needed):

```
rta --add actionlint
```

### update all applications

Update all configured applications to their latest versions:

```
rta --update
```

### globally installed applications

_Run-that-app_ can reuse tools already installed on your system. The executable
must be present in PATH, and the version must be declared as `system`.

```
go system 1.21.3
```

This prefers the system-installed Go. If none is found, Go 1.21.3 is installed
and used.

You can restrict acceptable versions for globally installed app:

```asdf
go system@1.21.* 1.21.3
```

### external version declarations

Some tools define their version in project files (e.g. Go via `go.mod`). Setting
the version to `auto` enables automatic detection:

```
go auto
```

## bundled applications

Some tools are distributed as part of another toolchain. In thes cases, specify
the version of the _bundling_ application.

### npm and npx

`npm` and `npx` are provided by Node.js. To use them, specify a Node version:

```asdf
npm 20.10.0
```

To run an `npm` that is already installed, provide its own version:

```asdf
npm system@10.2
```

You can combine both declarations:

```asdf
npm system@10.2 20.10.0
```

This prefers an existing `npm` ≥ 10.2, otherwise installs Node 20.10.0 and uses
the npm version that comes with it.

### gofmt

_Gofmt_ is bundled with Go. Specify the Go version:

```asdf
gofmt 1.21.6
```

This installs Go 1.21.6 and uses its bundled `gofmt`.

## Usage in a Makefile

Example Makefile integration:

```make
RTA_VERSION = 0.24.2  # version of run-that-app to use

# an example Make target that uses run-that-app
test: tools/rta@${RTA_VERSION}
	tools/rta actionlint

# this Make target installs run-that-app if it isn't installed or has the wrong version
tools/rta@${RTA_VERSION}:
	@rm -f tools/rta*
	@mkdir -p tools
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh)
	@mv tools/rta tools/rta@${RUN_THAT_APP_VERSION}
	@ln -s rta@${RUN_THAT_APP_VERSION} tools/rta
```

Add `tools/rta*` to `.gitignore`.

### Q&A

### Run-that-app does not support an application I need

Adding a new application is straightforward. See the
[developer documentation](docs/DEVELOPMENT.md).

### Why not use the package manager?

If it works for you, do it. In practice, package managers introduce issues:

- Different OSes use different package managers. You would need to support
  Homebrew, Nix, Scoop, Chocolatey, winget, DNF, pacman, apt, pkg, snap, zypper,
  xbps, portage, etc.
- Some environments like Windows or bare-bones Docker images have a package
  manager.
- Not all tools are packaged everywhere.
- Packaged application versions are often out of your control.
- Different projects often require different tool versions that would need to be
  installed in parallel.

### Why not use Docker?

Docker solves a different problem: shipping full runtime environments. For
development tooling, it often adds unnecessary complexity and bloat:

- extra OS layers (especially on Windows and macOS)
- significant storage and memory overhead
- Docker-in-Docker issues on CI
- no help with CPU architecture mismatches
- no solution for binaries hosted on GitHub releases

### Why not quickly write a small Bash script that downloads the executable?

Cross-platform Bash scripts quickly become fragile:

- they depend on external tools (`curl`, `tar`, `zip`)
- they and the external tools can behave differently across systems
- they don't work natively on Windows

Run-that-app is effectively a cross-platform Bash script, written in a strongly
typed programming language with predictable behavior.

### An app is not available for my platform

_Run-that-app_ can compile from source. If that fails, it can
[gracefully degrade](#graceful-degredation).

### What about NodeJS, Python, or Ruby tools?

_Run-that-app_ can execute the package managers and runtimes for these
ecosystems, which you can then use to execute tools written in these languages.

### An app has complex dependencies

Open an issue. Many cases are solvable.

### Why does run-that-app not have a marketplace that I can submit my application to?

That marketplace is _run-that-app's_ source code on GitHub. This has several
advantages.

1. You don't need to articulate complex installation and execution requirements
   and dependencies in some data format like JSON or YML, but can use a proper
   strongly-typed programming language. This gives you type checking (not just
   basic JSON-Schema linting), intelligent auto-completion, much more
   flexibility in how you implement downloading and unpacking archives or
   installing an application in other ways, and the ability to verify the
   installation using automated tests.

2. Having a separate marketplace would result in two separate systems that are
   versioned independently of each other: the version of _run-that-app_ and the
   version of the marketplace. Two separate versions lead to problems like an
   older versions of _run-that-app_ not able to work with newer versions of the
   marketplace. This severely limits how the data format of the marketplace can
   evolve. An embedded marketplace does not have this problem. _Run-that-app_
   can make breaking changes to the marketplace data at any time, and older
   installations will keep working.

   This makes _run-that-app_ a great tool to distribute third-party applications
   in locked-down environments. Only the hard-coded applications and versions
   can be installed.

3. If _run-that-app_ would use an external marketplace, it needs to sync its
   local replica of that marketplace at each invocation, and sometimes download
   updates. This introduces delays that might be acceptable for package managers
   that get called once to install an app, but not for an app runner that gets
   called a lot to execute the apps directly.

4. Even with an external marketplace, you would still need to update the
   _run-that-app_ executable regularly. So why not just do that and save
   yourself the hassle to also update a separate marketplace.

## Related solutions

These other cross-platform package managers might be a better fit for your use
case.

### asdf

[Asdf](https://asdf-vm.com) is the classic cross-platform application runner. It
is a mature and stable platform that installs a large variety of applications.
You load asdf plugins that tell asdf how to install applications. It can create
global or local shims for installed applications. Downsides of asdf are that it
is written in Bash, which makes it
[slow](https://github.com/asdf-vm/asdf/issues/290) and non-portable to Windows.

Compared to asdf, _run-that-app_ also supports Windows, offers conditional
execution, allows writing application installation logic in a robust programming
language that eliminates most runtime errors, and is faster.

### mise

[Mise](https://github.com/jdx/mise) is a rewrite of asdf in Rust. It allows
installing applications, sets up shims and shell integration. It also runs tasks
and manages your environment variables.

Compared to mise, _run-that-app_ is much simpler, works better in locked-down
environments, and focuses on doing one thing well.

### pkgx

[Pkgx](https://pkgx.sh) is a more full-fledged alternative to _run-that-app_
with additional bells and whistles, a better user experience, better shell
integration, and more polished design. It comes with its own
[app store](https://tea.xyz) that apps need to be listed in to be installable.
There is (or at least used to be) a blockchain component to this.

Compared to pkgx, _run-that-app_ is focused on doing one thing well, offers
additional features like the ability to compile from source, optional execution,
and checking whether an application is available for your platform.
