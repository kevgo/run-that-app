# Run That App!

[![linux](https://github.com/kevgo/run-that-app/actions/workflows/ci_linux.yml/badge.svg)](https://github.com/kevgo/run-that-app/actions/workflows/ci_linux.yml)
[![windows](https://github.com/kevgo/run-that-app/actions/workflows/ci_windows.yml/badge.svg)](https://github.com/kevgo/run-that-app/actions/workflows/ci_windows.yml)

> You don't want to install apps, you want to run them!

_Run-that-app_ executes CLI applications without the need to install them first.
This reduces boilerplate code during software development, DevOps, and on CI
servers.

_Run-that-app_ runs on all major computer platforms including Linux, Windows,
macOS, and BSD. Application downloads happen in 1-2 seconds, don't require
_sudo_, and store nothing but the executables on your hard drive. Run-that-app
can download and extract `.zip`, `.tar.gz`, and `.tar.xz` files on its own.

### quickstart

#### on Linux or macOS

1. Install the run-that-app executable:

   ```bash
   curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh
   ```

2. Run an app (in this case the GitHub CLI at version 2.39.1)

   ```bash
   ./run-that-app gh@2.39.1
   ```

#### on Windows Powershell

1. Download the run-that-app executable:

   ```powershell
   Invoke-Expression (Invoke-WebRequest -Uri "https://raw.githubusercontent.com/kevgo/run-that-app/main/download.ps1" -UseBasicParsing).Content
   ```

2. Run an app (in this case actionlint at version 1.6.26)

   ```batchfile
   .\run-that-app actionlint@1.6.26
   ```

#### installing into a specific directory

The installer script places the run-that-app executable into the current
directory. To install into a specific directory, change into that directory and
then execute the installer from there.

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
./run-that-app gh
```

### usage

```bash
run-that-app [run-that-app options] <app name>[@<app version override>] [app options]
```

Options for run-that-app come before the name of the application to run. All CLI
arguments after the application name are passed to the application.

Options:

- `--available`: signal via exit code whether an app is available on the local
  platform
- `--include-global`: if there is no pre-compiled binary for your platform, but
  a similarly named binary in your PATH, run the latter.
- `--log`: enable all logging
- `--log=domain`: enable logging for the given domain
  - see the available domains by running with all logging enabled
- `--optional`: if there is no pre-compiled binary for your platform, do
  nothing. This is useful for non-essential tools where it's okay if the tool
  doesn't run.
- `--show-path`: don't run the app but display the path to its executable

### examples

Runs [ShellCheck](https://shellcheck.net) version 0.9.0 with the arguments
`--color=always myscript.sh`.

```bash
run-that-app shellcheck@0.9.0 --color=always myscript.sh
```

#### Ignore unavailable applications

ShellCheck is just a linter. If it isn't available on a particular platform, the
tooling shouldn't abort with an error but simply skip ShellCheck.

```bash
run-that-app --ignore-unavailable shellcheck@0.9.0 --color=always myscript.sh
```

#### Using run-that-app as an installer

This example calls `go vet` with `alphavet` as a custom vet tool. But only if
`alphavet` is available for the current platform.

```bash
run-that-app --available alphavet && go vet "-vettool=$(run-that-app --show-path alphavet)" ./...
```

#### Usage in a Makefile

If you use `Make` to build your application and execute dev tools, here is a
template for installing and using run-that-app:

```make
RUN_THAT_APP_VERSION = 0.2.1

# an example Make target that calls run-that-app
test: tools/run-that-app@${RUN_THAT_APP_VERSION}
	tools/rta actionlint

# this Make target installs run-that-app if it isn't installed or has the wrong version
tools/run-that-app@${RUN_THAT_APP_VERSION}:
	@rm -f tools/run-that-app* tools/rta
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh)
	@mv tools/run-that-app tools/run-that-app@${RUN_THAT_APP_VERSION}
	@ln -s run-that-app@${RUN_THAT_APP_VERSION} tools/rta
```

### Q&A

#### Why not use the package manager of my system to install third-party applications?

If it works then do it. But keep in mind:

- Other people might use other operating systems that have other package
  managers like Homebrew, Nix, Scoop, Chocolatey, winget, DNF, pacman, apt, pkg,
  snap, zypper, xbps, portage, etc.
- Some environments like Windows or bare-bones Docker images might not have a
  package manager available.
- Some of the tools you use might not be available via every package manager. An
  example are tools distributed via GitHub Releases.
- You might need tooling a different version of an application than the one
  provided by your or other people's package manager. A best practice for
  reproducible builds is using tooling at an exactly specified version instead
  of whatever version your package manager gives you.
- You might work on several projects, each project requiring different versions
  of tools.
- Run-that-app saves you from having to write and maintain Bash and PowerShell
  scripts to install your dependencies and deal with various versions and
  flavors of `curl`, `gzip`, and `tar`

### Why not use Docker?

Docker is overkill for running simple applications that don't need a custom
Linux environment. Docker isn't available natively on macOS and Windows. Docker
often uses Gigabytes of hard drive space. Docker doesn't help with different CPU
architectures (Intel, ARM, Risc-V). Using Docker on CI can cause the
Docker-in-Docker problem.

#### What if an app does not distribute binaries for my platform?

Run-that-app can compile applications from source. If that doesn't work, it can
skip non-essential applications like linters via the `--ignore-unavailable`
switch.

#### What if I compile an app myself?

Run-that-app can build apps from source for you. If you want to do it yourself,
add the app that you compiled to the PATH and call _run-that-app_ with the
`--include-global` switch to make it run your app. In this case _run-that-app_
does not guarantee that the app has the correct version.

#### What about apps is written in NodeJS, Python, or Ruby?

Use the tooling of those frameworks to run that app!

#### What if my app has dependencies that run-that-app doesn't support?

Use Docker or WASI.
