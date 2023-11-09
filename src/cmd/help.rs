use crate::ui::Output;

pub fn help(output: &dyn Output) {
    print_usage(output);
    print_options(output);
    print_examples(output);
    // print_installable_apps();
}

fn print_usage(output: &dyn Output) {
    output.println("Usage: binstall install [options] application@version\n");
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
    output
        .println("\"binstall gh@2.34.0\" installs https://github.com/cli/cli at version 2.34.0\n");
}

// pub fn print_installable_apps() {
//     println!("\nInstallable applications:");
//     let app_manager = apps::Manager::new();
//     let installables = app_manager.installable_apps();
//     let max_width = longest_name(&installables) + 1;
//     for app in app_manager.installable_apps() {
//         println!("{:max_width$} {}", app.name, app.repo);
//     }
//     println!("\nRequest additional apps at https://github.com/kevgo/binstall/issues.");
// }

// fn longest_name(apps: &[InstallableApp]) -> usize {
//     apps.iter()
//         .fold(0, |result, app| result.max(app.name.len()))
// }
