# Run That App!

[![linux](https://github.com/kevgo/run-that-app/actions/workflows/ci_linux.yml/badge.svg)](https://github.com/kevgo/run-that-app/actions/workflows/ci_linux.yml)
[![windows](https://github.com/kevgo/run-that-app/actions/workflows/ci_windows.yml/badge.svg)](https://github.com/kevgo/run-that-app/actions/workflows/ci_windows.yml)

> Minimalistic cross-platform application runner

_Run-that-app_ executes native CLI applications on Linux, Windows, macOS, and
BSD without the need to install them first. Installation across all possible
operating systems is a complex and nuanced problem without a good solution.
Run-that-app bypasses this problem.

Run-that-app is minimalistic and completely non-invasive. It ships as a single
stand-alone binary. Run-that-app uses no magic, no configuration changes, no
environment variables, no application shims or stubs, no shell integrations, no
dependencies, no plugins, no need to package applications to install in a
special way, no application repository, no Docker, no system daemons, no sudo,
no emulation, no WASM, no bloat. Applications download in 1-2 seconds, and store
very little (just the executables) on your hard drive. Applications execute at
100% native speed.

### quickstart

#### on Linux or macOS

1. Install the run-that-app executable:

   ```bash
   curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh
   ```

2. Run an app (in this case [actionlint](https://github.com/rhysd/actionlint) at
   version 1.6.26)

   ```bash
   ./rta actionlint@1.6.26
   ```

#### on Windows (Powershell)

1. Download the run-that-app executable:

   ```powershell
   Invoke-Expression (Invoke-WebRequest -Uri "https://raw.githubusercontent.com/kevgo/run-that-app/main/download.ps1" -UseBasicParsing).Content
   ```

2. Run an app (in this case [actionlint](https://github.com/rhysd/actionlint) at
   version 1.6.26)

   ```batchfile
   .\rta actionlint@1.6.26
   ```

#### installing the run-that-app executable into a specific directory

The installer script places the run-that-app executable into the current
directory. To install in another directory, change into that directory and then
execute the installer from there.

### configuration

You can configure the versions of applications that run-that-app should use in a
[.tool-versions](https://asdf-vm.com/manage/configuration.html) file that looks
like this:

```
actionlint 1.6.26
shellcheck 0.9.0
```

Now you can run these applications without having to provide their version
numbers:

```bash
rta actionlint
```

Executing `rta --setup` creates a template of this file for you.

### usage

```bash
rta [run-that-app arguments] <app name>[@<app version override>] [app arguments]
```

Arguments for run-that-app come before the name of the application to run. The
application name is the first CLI argument that doesn't start with a dash. All
CLI arguments after the application name are passed to the application.

Run-that-app Arguments:

- `--available`: signal via exit code whether an app is available on the local
  platform
- `--error-on-output`: treat all output of the executed application as an error
  condidion
- `--help` or `-h`: show help screen
- `--log`: enable all logging
- `--log=domain`: enable logging for the given domain
  - see the available domains by running with all logging enabled
- `--optional`: if there is no pre-compiled binary for your platform, do
  nothing. This is useful for non-essential applications that shouldn't break
- `--update`: updates the versions in `.tool-versions` automation if they are
  not available.
- `--which`: displays the path to the installed executable of the given
  application
- `--version` or `-V`: displays the version of run-that-app
- `--versions=<number>`: displays the given amount of most recent versions of
  the given app

The app version override should consist of just the version number, i.e.
`1.6.26` and not `v1.6.26`.

### examples

Runs [ShellCheck](https://shellcheck.net) version 0.9.0 with the arguments
`--color=always myscript.sh`.

```bash
rta shellcheck@0.9.0 --color=always myscript.sh
```

#### Ignore unavailable applications

ShellCheck is just a linter. If it isn't available on a particular platform, the
tooling shouldn't abort with an error but simply skip ShellCheck.

```bash
rta --optional shellcheck@0.9.0 --color=always myscript.sh
```

#### Access the installed executables

This example calls `go vet` with `alphavet` as a custom vet tool. But only if
`alphavet` is available for the current platform.

```bash
rta --available alphavet && go vet "-vettool=$(rta --which alphavet)" ./...
```

#### Usage in a Makefile

Here is a template for installing and using run-that-app in a `Makefile`:

```make
RTA_VERSION = 0.4.1

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

You would have to `.gitignore` the files `tools/rta*`.

### npm and npx

_Run-that-app_ executes the `npm` and `npx` executables that come with the
Node.js installation. Hence, to install them, you need to provide the Node
version. To use already installed executables in your PATH, you need to provide
the versions of `npm` and `npx`.

Example _.tool-versions_ for npm:

```asdf
npm system@10.2 20.10.0
```

This tries to use an existing npm installation as long as it has version 10.2 or
higher. If your machine has no npm installed, this installs Node 20.10.0 and
uses the npm version that comes with it.

### gofmt

_Gofmt_ is distributed as part of a Go installation. So please provide the Go
version when specifying the desired gofmt version. Example _.tools-versions_
file:

```asdf
gofmt 1.21.6
```

This installs Go 1.21.6 and calls the gofmt contained in this installation.

### Q&A

#### Why not use the package manager of my system to install third-party applications?

If it works then do it. We have found many challenges with using executables
installed by package managers:

- Other people might use other operating systems that have other package
  managers. You would have to support Homebrew, Nix, Scoop, Chocolatey, winget,
  DNF, pacman, apt, pkg, snap, zypper, xbps, portage, etc.
- Some environments like Windows or bare-bones Docker images might not have a
  package manager available.
- Some of the tools you use might not be available via every package manager. An
  example are tools distributed via GitHub Releases.
- You might need a different version of an application than the one provided by
  your or other people's package manager. A best practice for reproducible
  builds is using tooling at an exactly specified version instead of whatever
  version your package manager gives you on a particular day.
- You might work on several projects, each project requiring different versions
  of tools.

#### Why not use Docker?

Docker is overkill for running simple applications that don't need a custom
Linux environment. Docker isn't available natively on macOS and Windows. Docker
often uses Gigabytes of hard drive space. Docker doesn't help with different CPU
architectures (Intel, ARM, Risc-V). Using Docker on CI can cause the
Docker-in-Docker problem.

#### Why not quickly write a small Bash script that downloads the executable?

These Bash scripts tend to become complex if you want them to work well on a
variety of operating systems. They require additional applications like `curl`,
`gzip`, and `tar`, which must exist on all machines that your Bash script runs
on. Bash itself as well as these external dependencies come in a variety of
versions and flavors that sometimes aren't compatible with each other.

You also need to write a Powershell script since Bash isn't available
out-of-the-box on Windows. Even if Bash is installed on Windows, it executes in
an emulated environment that behaves different than a real Linux or Unix system.

Run-that-app saves you from these headaches.

#### What if an app does not distribute binaries for my platform?

Run-that-app can compile applications from source. If that doesn't work, it can
skip non-essential applications like linters via the `--optional` switch.

#### What if I compile an app myself?

Add the app that you compiled to the PATH and add a "system" version in the
configuration file that looks like this:

```asdf
acme 1.2.3 system
```

This tries to first install and run the app named `acme` at version 1.2.3. If
this is not successful, _run-that-app_ looks for an application named `acme` in
the PATH and executes it instead. In this case _run-that-app_ does not guarantee
that the app has the correct version.

You can restrict the acceptable versions of the globally installed applications
like this:

```asdf
acme 1.2.3 system@1.2.*
```

This tries to first install and run the app named `acme` at version 1.2.3. If
this is not successful, _run-that-app_ looks for an application named `acme` in
the PATH, determines its version, and if that version matches the given semver
restrictions, executes it. For example, if you have `acme` at version 1.2.1
installed somewhere in your PATH, _run-that-app_ would execute it.

#### What about apps is written in NodeJS, Python, or Ruby?

Use the tooling of those frameworks to run that app!

#### What if my app has dependencies that run-that-app doesn't support?

Use Docker or WASI.

### Related solutions

These other cross-platform package managers might be a better fit for your use
case.

#### asdf

[Asdf](https://asdf-vm.com) is the classic cross-platform application runner. It
is a mature and stable platform that installs a large variety of applications.
You load asdf plugins that tell asdf how to install applications. It can create
global or local shims for installed applications. Downsides of asdf are that it
is written in Bash, which makes it
[slow](https://github.com/asdf-vm/asdf/issues/290) and non-portable to Windows.

Compared to asdf, run-that-app also supports Windows, offers conditional
execution, allows writing application installation logic in a robust programming
language that eliminates most runtime errors, and is faster.

#### mise

[Mise](https://github.com/jdx/mise) is a rewrite of asdf in Rust. It allows
installing applications, sets up shims and shell integration.

Compared to rtx, run-that-app also supports Windows, offers conditional
execution, and allows writing application installation logic in a robust
programming language that eliminates most runtime errors.

#### pkgx

[Pkgx](https://pkgx.sh) is a more full-fledged alternative to run-that-app with
more bells and whistles, a better user experience, better shell integration, and
more polished design. It comes with its own [app store](https://tea.xyz) that
apps need to be listed in to be installable. These is (or at least used to be) a
blockchain component to this.

Compared to pkgx, run-that-app is leaner, supports more platforms (Windows), and
offers additional features like the ability to compile from source, optional
execution, and checking whether an application is available for your platform.
