# Run That App!

_Run-That-App!_ installs and executes small third-party tools used by software developers (linters, checkers , verifiers) that often have no good way of being installed.
Like Docker, _run-that-app!_ is cross-platform. It works on Linux, Windows, and macOS.
Unlike Docker, _run-that-app!_ is extremely lean, fast, has zero dependencies and leave a very small footprint on your computer.

### show me

```
# download the _run-that-app!_ executable
curl https://raw.githubusercontent.com/kevgo/run-that-app/main/install.sh | sh

# run an app through _run-that-app!_, in this case ShellCheck
./run-that-app shellcheck
```

### how it works

When running an application, _run-that-app!_ determines your platform (operating system, CPU architecture) and downloads the right binary for your platform. This typically takes a second or less. It then stores the downloaded app under `.run-that-app/apps/<name>/<version>/<executable>` on your hard drive and executes it.
The next time you run the same application through _run-that-app!_, it recognizes that the app is already downloaded and calls it right away.

### usage

```
run-that-app [run-that-app options] <app name> [app options]
```

Example: run ShellCheck v0.9.0. If there is no binary available for your platform, try running a manually installed version that is in the PATH. If no such app exists, do nothing and don't return an error (ShellCheck is just a verifier and if it cannot run, that's not an error condition):

```
run-that-app --fallback-to-global --allow-unavailable shellcheck@0.9.0 --color=always myscript.sh
```

### Q&A

Q: What if an app does not provide binaries for my platform?

A: If the app is not essential, you can provide the `--allow-unavailable` switch and _run-that-app!_ will simply do nothing, including not exiting with an error.

Q: What if I compiled an unavailable app by hand?

A: Add your version to the PATH and call _run-that-app!_ with the `--use-global-app` switch to tell it to run your app. In this case _run-that-app!_ does not guarantee that the app has the correct version.

Q: What about apps that require a dependency like NodeJS or Python?

A: These platforms already have a package manager that allows installing and executing apps. Use that package manager to run that app!
