<br>
<div align="center">
<img alt="Run that app logo" src="docs/logo.png" width="725" height="177" align="center">
</div>

<br><br>

_Run-that-app_ is a minimalistic cross-platform application runner. It executes
native CLI applications on Linux, macOS, Windows, and BSD without the need to
install them first. The intended use case is running developer tools like
linters in scripts.

#### integrating installation and execution

Installation of small developer tools at specific versions across all possible
operating systems is a complex problem without a good solution. Run-that-app
bypasses this problem. You don't really want to _install_ applications, what you
actually want is to _run_ them in your development workflows.

#### radically minimalistic

Run-that-app is minimalistic and completely non-invasive. It ships as a single
stand-alone binary. Following the principle "perfection is not achieved when
there is nothing left to add, but when there is nothing left to take away",
run-that-app uses no environment variables, no application shims, no shell
integrations, no dependencies, no plugins, no need to package applications into
a specific container format, no need to install applications, no application
repository, no Docker, no WASM, no system daemons, no sudo, no emulation, no IDE
plugins, no bloat. Applications download in 1-2 seconds from their original
hosting location, and store very little (just the executables) on your hard
drive. Applications execute at 100% native speed.

[![linux](https://github.com/kevgo/run-that-app/actions/workflows/ci_linux.yml/badge.svg)](https://github.com/kevgo/run-that-app/actions/workflows/ci_linux.yml)
[![windows](https://github.com/kevgo/run-that-app/actions/workflows/ci_windows.yml/badge.svg)](https://github.com/kevgo/run-that-app/actions/workflows/ci_windows.yml)

## installation

Linux and macOS:

```bash
curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh
```

Windows (Powershell):

```powershell
Invoke-Expression (Invoke-WebRequest -Uri "https://raw.githubusercontent.com/kevgo/run-that-app/main/download.ps1" -UseBasicParsing).Content
```

The installer script places the run-that-app executable into the current
directory. To install in another directory, execute the installer inside it.

## usage

```bash
rta [run-that-app arguments] <app name>[@<app version override>] [app arguments]
```

Run [actionlint](https://github.com/rhysd/actionlint) at version 1.6.26:

```bash
./rta actionlint@1.6.26
```

The app version should consist of just the version number, i.e. `1.6.26`, even
if the Git tag is `v1.6.26`.

Arguments for run-that-app come before the name of the application to run. The
application name is the first CLI argument that doesn't start with a dash. All
CLI arguments after the application name are passed to the application. Here we
run [ShellCheck](https://shellcheck.net) version 0.9.0 with the arguments
`--color=always myscript.sh`.

```bash
rta shellcheck@0.9.0 --color=always myscript.sh
```

### see all runnable applications

```sh
rta --apps
```

### graceful degredation

Not all applications support all platforms. The `--optional` flag skips such
applications without causing errors.

This runs ShellCheck only if it is available on your machine:

```bash
rta --optional shellcheck@0.9.0 myscript.sh
```

The `--available` command indicates via exit code whether an application is
available on the current platform.

### get the path to the installed executable

The `--which` command returns the path to the installed executable.

Here we call `go vet` with `alphavet` as a custom vet tool. If `alphavet` is
unavailable for the current platform, do nothing.

```bash
rta --available alphavet && go vet "-vettool=$(rta --which alphavet)" ./...
```

### error-on-output

Some linters like [deadcode](https://pkg.go.dev/golang.org/x/tools/cmd/deadcode)
print findings but don't signal failure with an exit code. In this case, the
`--error-on-output` flag makes _run-that-app_ monitor the output of the
application it calls and exit will an error if there is any.

```
rta --error-on-output deadcode
```

### see available versions

To see which versions of an applications you can install:

```
rta --versions actionlint
```

This prints the 10 most recent versions. To see a different number:

```
rta --versions=3 actionlint
```

### force installation from source

If the application you want to run provides precompiled binaries, for example on
GitHub releases, _run-that-app_ tries to download and use them. If no binaries
are available for your platform, _run-that-app_ can also compile applications
from source.

You can enforce compilation from source with the `--from-source` flag.

## configuration

You can configure the versions of applications to use in a file called
`run-that-app` that follows the
[asdf format](https://asdf-vm.com/manage/configuration.html):

```
actionlint 1.6.26
shellcheck 0.9.0
```

Now you can run the listed applications without having to provide their version
numbers:

```bash
rta actionlint
```

_Run-that-app_ uses a different name for the configuration file to avoid
interference with other app runners like [asdf](#asdf) or [mise](#mise).

### add an application

This command creates the _run-that-app_ configuration file if necessary and adds
the `actionlint` application at the latest version:

```
rta --add actionlint
```

### update all applications

The `--update` command updates all applications listed in the configuration file
to the latest version:

```
rta --update
```

### globally installed applications

If your system already has certain apps installed, _run-that-app_ can use them.
Consider this `run-that-app` file:

```
go system 1.21.3
```

If your computer has Go installed, _run-that-app_ would try to run it. Only if
that fails would it install and run Go version 1.21.3.

_Run-that-app_ considers restrictions declared by your code base. If your
codebase has a file `go.mod` containing `go 1.21` and the externally installed
Go version is older, _run-that-app_ would not use the external version.

### use externally defined versions

Certain applications allow defining the version to use in their own config file.
An example is Go, which defines the Go version to use in the `go.mod` file.
Setting the version to `auto` makes _run-that-app_ auto-detect these versions.
Example config file entry:

```
go auto
```

## bundled applications

Some tools are shipped bundled with other tools. When running them, provide the
version of the bundling application.

### npm and npx

_Run-that-app_ executes the `npm` and `npx` executables that come with the
Node.js installation. To use them, provide the Node version.

Example _run-that-app_ config file for npm bundled with Node 20.10.0:

```asdf
npm 20.10.0
```

If you already have `npm` installed in your PATH, provide the versions of `npm`
and `npx` themselves.

```asdf
npm system@10.2
```

You can combine these two declarations:

```asdf
npm system@10.2 20.10.0
```

This tries to use an existing npm installation as long as it has version 10.2 or
higher. If your machine has no npm installed, _run-that-app_ installs Node
20.10.0 and uses the npm version that comes with it.

### gofmt

_Gofmt_ is distributed as part of a Go installation. So please provide the Go
version when specifying the desired gofmt version. Example _run-that-app_ config
file:

```asdf
gofmt 1.21.6
```

This installs Go 1.21.6 and calls the gofmt contained in this installation.

## Usage in a Makefile

Here is a template for installing and using run-that-app in a `Makefile`:

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

You would have to `.gitignore` the files `tools/rta*`.

### Q&A

#### Run-that-app does not support an application I need

It's super easy to add a new application to _run-that-app_! See `DEVELOPMENT.md`
for details.

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

Docker is a standardized container format for distributing complex applications
along with their runtime environments and dependencies. Its benefits are often
likened to those of standardized shipping containers in maritime transport: a
reliable, consistent, and portable way to package and deliver goods, or, in this
case, software.

However, just as companies rarely manufacture goods inside shipping containers,
you likely don't need to "manufacture" your software inside a container either.
Your development machine already has a capable operating system, there’s no need
to layer additional OS environments just to write and debug code if the required
tooling is available for your native operating system.

Consider the implications: on macOS or Windows, which lack native Docker
support, you end up running a full Linux instance inside a virtual machine,
which then runs another Linux environment inside Docker. That’s two and a half
operating systems in play! Each additional OS layer consumes gigabytes of
storage and RAM, adding unnecessary complexity and overhead to your workflow.

Moreover, Docker falls short in key areas. It doesn’t resolve compatibility
issues with different CPU architectures (Intel, ARM, RISC-V). Using Docker in CI
can introduce the infamous "Docker-in-Docker" problem. And if you need to
install arbitrary executables from GitHub Releases, for which there are no
Docker images available, Docker won’t make that process any easier.

#### Why not quickly write a small Bash script that downloads the executable?

These Bash scripts tend to become complex if you want them to work well on a
variety of operating systems. They require additional applications like `curl`,
`gzip`, and `tar`, which must exist on all machines that your Bash script runs
on. Bash itself as well as these external dependencies come in a variety of
versions and flavors that sometimes aren't compatible with each other.

You also need to write a Powershell script since Bash isn't available
out-of-the-box on Windows. Even if Bash is installed on Windows, it executes in
an emulated environment that behaves different than a real Linux or Unix system.

Run-that-app saves you from these headaches. Think about it as a cross-platform
Bash script, written in a strongly typed language that guarantees correctness.

#### What if an app does not distribute binaries for my platform?

Run-that-app can compile applications from source. If that doesn't work, it can
skip non-essential applications like linters via the `--optional` flag.

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

Use the package managers of those frameworks to run that app.

#### What if my app has more complex dependencies that run-that-app cannot support?

Please open a ticket to discuss your use case. Run-that-app can install these
dependencies.

#### Why does run-that-app not have a marketplace that I can submit my application to?

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

3. If _run-that-app_ would use an external marketplace, it needs to sync its
   local replica of that marketplace at each invocation, and sometimes download
   updates. This introduces delays that might be acceptable for package managers
   that get called once to install an app, but not for an app runner that gets
   called a lot to execute the apps directly.

4. Even with an external marketplace, you would still need to update the
   _run-that-app_ executable regularly. So why not just do that and save
   yourself the hassle to also update a separate marketplace.

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
installing applications, sets up shims and shell integration. It also runs tasks
and manages your environment variables.

Compared to mise, run-that-app is much simpler and focused on doing one thing
well.

#### pkgx

[Pkgx](https://pkgx.sh) is a more full-fledged alternative to run-that-app with
more bells and whistles, a better user experience, better shell integration, and
more polished design. It comes with its own [app store](https://tea.xyz) that
apps need to be listed in to be installable. There is (or at least used to be) a
blockchain component to this.

Compared to pkgx, run-that-app is focused on doing one thing well, offers
additional features like the ability to compile from source, optional execution,
and checking whether an application is available for your platform.
