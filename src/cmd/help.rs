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
--optional                       if an app is not available for the current platform, create a stub that does nothing
--include-path                   if an app is not available but one is in the PATH, execute that one
--update                         updates the versions in .tool-versions to the latest available
--available                      indicates via the exit code whether the given application is available on this platform
--which                          displays the path to the installed executable of the given application
--log, -l                        enable logging of all categories
--log=<category>, -l=<category>  enable logging for the given category
--help, -h                       display this help screen
",
    );
}

fn print_examples() {
    println!("Examples:");
    println!("\"rta gh@2.34.0\" installs https://github.com/cli/cli at version 2.34.0\n");
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
