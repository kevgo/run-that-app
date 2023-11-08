# Run That App!

_Run-that-app!_ installs and executes small third-party tools used by software
developers (linters, checkers , verifiers) that often have no good way of being
installed. Like Docker, _run-that-app!_ works on Linux, Windows, and macOS.
Unlike Docker, _run-that-app!_ is extremely lean, fast, has zero dependencies,
works inside Docker, and leaves a very small footprint on your computer.

### show me

```
# download the _run-that-app!_ executable
curl https://raw.githubusercontent.com/kevgo/run-that-app/main/install.sh | sh

# run an app (in this case ShellCheck)
./run-that-app shellcheck@0.9.0
```

### why

Reasons to use `run-that-app` over traditional forms of installation (package
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

When running a third-party application, _run-that-app!_:

- determines your platform (operating system, CPU architecture)
- downloads and unpacks the matching executable for your platform - this
  typically takes just a second or two for most applications
- stores the downloaded executable under `~/.run-that-app` on your hard drive
- executes this binary

### usage

```
run-that-app [run-that-app options] <app name> [app options]
```

### examples

```
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

Q: What if an app does not provide binaries for my platform?

A: If the app is not essential, you can provide the `--allow-unavailable` switch
and _run-that-app!_ will simply do nothing, including not exiting with an error.

Q: What if I compile an app that doesn't provide binaries for my platform
myself?

A: Add your version to the PATH and call _run-that-app!_ with the
`--use-global-app` switch to make it run your app. In this case _run-that-app!_
does not guarantee that the app has the correct version.

Q: What about apps that require a dependency like NodeJS or Python?

A: Use the package manager of those platforms to run that app!
