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
    println!("Usage: run-that-app install [options] application@version\n");
}

pub fn print_options() {
    println!(
        "
Options:
--ignore-unavailable             if an app is not available for the current platform, create a stub that does nothing
--include-global                 if an app is not available for the current platform, use the globally installed app if one exists
--update                         updates the versions in .tool-versions to the latest available
--available                      indicates via the exit code whether the given application is available on this platform
--show-path                      displays the path to the installed executable of the given application
--log, -l                        enable logging of all categories
--log=<category>, -l=<category>  enable logging for the given category
--help, -h                       display this help screen
",
    );
}

fn print_examples() {
    println!("Examples:");
    println!("\"run-that-app gh@2.34.0\" installs https://github.com/cli/cli at version 2.34.0\n");
}

pub fn print_installable_apps() {
    println!("\nInstallable applications:");
    let apps = apps::all();
    let max_width = apps.iter().map(|app| app.name().len()).max().unwrap() + 1;
    for app in apps {
        println!("{:max_width$} {}", app.name(), app.homepage());
    }
    println!("\nRequest additional apps at https://github.com/kevgo/run-that-app/issues.");
}
