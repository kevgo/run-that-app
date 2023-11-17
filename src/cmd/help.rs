use crate::{apps, Output};
use std::process::ExitCode;

pub fn help(output: &dyn Output) -> ExitCode {
    print_usage(output);
    print_options(output);
    print_examples(output);
    print_installable_apps(output);
    ExitCode::SUCCESS
}

fn print_usage(output: &dyn Output) {
    output.println("Usage: run-that-app install [options] application@version\n");
}

pub fn print_options(output: &dyn Output) {
    output.println(
        "
Options:
--allow-unavailable              if an app is not available for the current platform, create a stub that does nothing
--fallback-to-existing           if an app is not available for the current platform, use the globally installed app if one exists
--log, -l                        enable logging of all categories
--log=<category>, -l=<category>  enable logging for the given category
--help, -h                       display this help screen
",
    );
}

fn print_examples(output: &dyn Output) {
    output.println("Examples:");
    output.println(
        "\"run-that-app gh@2.34.0\" installs https://github.com/cli/cli at version 2.34.0\n",
    );
}

pub fn print_installable_apps(output: &dyn Output) {
    output.println("\nInstallable applications:");
    let apps = apps::all();
    let max_width = apps.iter().map(|app| app.name().len()).max().unwrap() + 1;
    for app in apps {
        output.println(&format!("{:max_width$} {}", app.name(), app.homepage()));
    }
    output.println("\nRequest additional apps at https://github.com/kevgo/run-that-app/issues.");
}
