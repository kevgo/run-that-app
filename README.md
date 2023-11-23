# Run That App!

[![linux](https://github.com/kevgo/run-that-app/actions/workflows/ci_linux.yml/badge.svg)](https://github.com/kevgo/run-that-app/actions/workflows/ci_linux.yml)
[![windows](https://github.com/kevgo/run-that-app/actions/workflows/ci_windows.yml/badge.svg)](https://github.com/kevgo/run-that-app/actions/workflows/ci_windows.yml)

_Run-that-app_ executes small third-party tools used by software developers
(linters, checkers, verifiers) without the need to install them first. This
reduces boilerplate code in developer tooling and on CI servers.

_Run-that-app_ runs on all major computer platforms including Linux, Windows,
and macOS. Installs happen in seconds, don't require _sudo_, and store nothing
but the executables on your hard drive. Run-that-app has zero dependencies (it
can download and extract `.zip`, `.tar.gz`, and `.tar.xz` files on its own) and
avoids the Docker-in-Docker problem.

### quickstart

#### on Linux or macOS

1. Download the run-that-app executable:

   ```bash
   curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh
   ```

2. Run an app (in this case the GitHub CLI)

   ```bash
   ./run-that-app gh@2.39.1
   ```

#### on Windows Powershell

1. Download the run-that-app executable:

   ```powershell
   Invoke-Expression (Invoke-WebRequest -Uri "https://raw.githubusercontent.com/kevgo/run-that-app/main/download.ps1" -UseBasicParsing).Content
   ```

2. Run an app (in this case the GitHub CLI)

   ```bash
   .\run-that-app gh@2.39.1
   ```

### why

Reasons to use _run-that-app_ over traditional forms of installation (package
managers) are:

- You need to run a binary for which no good way to install them exists, for
  example binaries distributed through GitHub Releases.
- You use or support a variety of operating systems and want to avoid having to
  deal with an endless variety of package managers like Homebrew, Nix, Scoop,
  Chocolatey, winget, DNF, pacman, apt, pkg, snap, zypper, xbps, portage, etc.
- You don't want to write and maintain Bash scripts to install your dependencies
  and deal with various versions and flavors of `curl`, `gzip`, and `tar`
- You work in an environment in which no package manager is available.
- You need a different version of an application than the one installed by your
  package manager.
- You work on multiple projects that require different versions of development
  tools.
- You want to avoid the overhead of Docker.
- You want to install third-party tools as fast as possible for the best
  developer experience.

### how it works

When running a third-party application, _run-that-app_:

- downloads and unpacks the matching executable for your platform - this
  typically takes just a second or two
- stores the downloaded executable under `~/.run-that-app` on your hard drive
- executes this binary

### usage

```bash
run-that-app [run-that-app options] <app name> [app options]
```

Options:

- `--ignore-unavailable`: if there is no pre-compiled binary for your platform,
  do nothing. This is useful for non-essential tools where it's okay if the tool
  doesn't run.
- `--include-global`: if there is no pre-compiled binary for your platform, but
  a similarly named binary in your PATH, run the latter.
- `--log`: enable all logging
- `--log=domain`: enable logging for the given domain
  - see the available domains by running with all logging enabled

### examples

Runs ShellCheck version 0.9.0 with the arguments `--color=always myscript.sh`.

```bash
run-that-app --include-global --ignore-unavailable shellcheck@0.9.0 --color=always myscript.sh
```

Same call but if ShellCheck is not available, do nothing.

```bash
run-that-app --ignore-unavailable shellcheck@0.9.0 --color=always myscript.sh
```

### Q&A

#### Why not use the package manager of my system to install third-party applications?

Do it if your and everybody else's package manager installs all needed tools at
versions that work for your use cases.

#### What if an app does not provide binaries for my platform?

If the app is not essential, you can provide the `--ignore-unavailable` switch
and _run-that-app_ will simply do nothing, including not exiting with an error.

#### What if I compile an app that doesn't provide binaries for my platform myself?

Add your version to the PATH and call _run-that-app_ with the `--include-global`
switch to make it run your app. In this case _run-that-app_ does not guarantee
that the app has the correct version.

#### What about apps that require a dependency like NodeJS or Python?

Use the package manager of those frameworks to run that app!
