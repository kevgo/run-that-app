# Run That App!

[![linux](https://github.com/kevgo/run-that-app/actions/workflows/ci_linux.yml/badge.svg)](https://github.com/kevgo/run-that-app/actions/workflows/ci_linux.yml)
[![windows](https://github.com/kevgo/run-that-app/actions/workflows/ci_windows.yml/badge.svg)](https://github.com/kevgo/run-that-app/actions/workflows/ci_windows.yml)

_Run-that-app_ executes small third-party tools used by software developers
(linters, checkers, verifiers) without the need to install them first. This
reduces boilerplate code in developer tooling and on CI servers.

_Run-that-app_ runs on all major computer platforms including Linux, Windows,
and macOS. Installs happen in seconds, don't require _sudo_, and store nothing
but the executables on your hard drive. Run-that-app has zero dependencies and
avoids the Docker-in-Docker problem.

### quickstart

1. Download the run-that-app executable:

   ```bash
   curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh
   ```

2. Run an app (in this case the GitHub CLI)

   ```bash
   ./run-that-app gh@2.39.1
   ```

### why

Reasons to use _run-that-app_ over traditional forms of installation (package
managers) are:

- You need to run a binary for which no good way to install them exists, for
  example binaries distributed via GitHub Releases.
- You use or support a wide variety of operating systems and want to avoid
  having to deal with an endless variety of package managers like Homebrew, Nix,
  Scoop, Chocolatey, winget, DNF, pacman, apt, pkg, snap, zypper, xbps, portage,
  etc.
- You don't want to write and maintain Bash scripts to install your dependencies
  and deal with various versions and flavors of `curl`, `gzip`, and `tar`
- You work in an environment in which no package manager is available.
- You need a different version of an application than the one installed by your
  package manager.
- You work on multiple projects that require different versions development
  tools.
- You want to avoid the overhead of working inside Docker.
- You want to install third-party tools as fast as possible for the best
  developer experience.

### how it works

When running a third-party application, _run-that-app_:

- determines your platform (operating system, CPU architecture)
- downloads and unpacks the matching executable for your platform - this
  typically takes just a second or two for most applications
- stores the downloaded executable under `~/.run-that-app` on your hard drive
- executes this binary

### usage

```bash
run-that-app [run-that-app options] <app name> [app options]
```

### examples

```bash
run-that-app --fallback-to-global --allow-unavailable shellcheck@0.9.0 --color=always myscript.sh
```

- Runs ShellCheck version 0.9.0 with the arguments `--color=always myscript.sh`.
- Because the `--fallback-to-global` option is enabled, if there is no binary
  available for your platform, _run-that-app_ looks for one in the PATH and runs
  that one.
- The `--allow-unavailable` switch makes _run-that-app_ do nothing if the app
  cannot be found. ShellCheck is just a linter and it's okay if it cannot run on
  a few exotic developer machines.

### Q&A

#### Wouldn't it be more appropriate to use the package manager of my system to install third-party applications?

Yes if your and everybody else's package manager installs a version that works
for all your use cases.

#### What if an app does not provide binaries for my platform?

If the app is not essential, you can provide the `--allow-unavailable` switch
and _run-that-app_ will simply do nothing, including not exiting with an error.

#### What if I compile an app that doesn't provide binaries for my platform myself?

Add your version to the PATH and call _run-that-app_ with the `--use-global-app`
switch to make it run your app. In this case _run-that-app_ does not guarantee
that the app has the correct version.

#### What about apps that require a dependency like NodeJS or Python?

Use the package manager of those platforms to run that app!
