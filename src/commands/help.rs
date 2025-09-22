use std::process::ExitCode;

pub(crate) fn help() -> ExitCode {
  print_usage();
  print_options();
  print_examples();
  ExitCode::SUCCESS
}

fn print_usage() {
  println!("Usage: rta install [options] application@version\n");
}

pub(crate) fn print_options() {
  println!(
    "OPTIONS:

--add <app>                 add the given application to the configuration file
--apps, -a                  display all installable applications
--available <app>           signal via exit code whether the given application is available on this platform
--error-on-output           treat all output of the executed app as an error
--from-source               force installation from source, even if precompiled binaries are available
--help, -h                  display this help screen
--optional                  if an app is not available for the current platform, do nothing
--update                    updates the versions in .run-that-app to the latest available
--which <app>               displays the path to the installed executable of the given application
--verbose, -v               display more details
--version, -V               displays the version of run-that-app
--versions <app>            displays the 10 most recent available versions of the given app
--versions=<number> <app>   displays the given number of most recent available versions of the given app
",
  );
}

fn print_examples() {
  println!("EXAMPLES:\n");
  println!("\"rta gh@2.34.0\" runs https://github.com/cli/cli at version 2.34.0\n");
}
