use crate::apps;
use std::process::ExitCode;

pub fn help() -> ExitCode {
    print_usage();
    print_options();
    print_examples();
    print_installable_apps();
    ExitCode::SUCCESS
}

fn print_usage() {
    println!("Usage: rta install [options] application@version\n");
}

pub fn print_options() {
    println!(
        "
Options:
--available                      indicates via the exit code whether the given application is available on this platform
--error-on-output                treat all output of the executed app as an error
--help, -h                       display this help screen
--include-path                   if an app is not available but one is in the PATH, execute that one
--log, -l                        enable logging of all categories
--log=<category>, -l=<category>  enable logging for the given category
--optional                       if an app is not available for the current platform, create a stub that does nothing
--update                         updates the versions in .tool-versions to the latest available
--which                          displays the path to the installed executable of the given application
--version, -V                    displays the version of run-that-app
--versions                       displays the 10 most recent available versions of the given app
--versions=<number>              displays the given number of most recent available versions of the given app
",
    );
}

fn print_examples() {
    println!("Examples:");
    println!("\"rta gh@2.34.0\" runs https://github.com/cli/cli at version 2.34.0\n");
}

pub fn print_installable_apps() {
    println!("\nInstallable applications:");
    let apps = apps::all();
    let max_width = apps.longest_name_length() + 1;
    for app in apps.iter() {
        println!("{:max_width$} {}", app.name(), app.homepage());
    }
    println!("\nRequest additional apps at https://github.com/kevgo/run-that-app/issues.");
}
