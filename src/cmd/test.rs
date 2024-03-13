use crate::install;
use crate::{apps, logger, platform, yard, Result};
use colored::Colorize;
use std::process::ExitCode;

pub fn test(verbose: bool) -> Result<ExitCode> {
    let apps = apps::all();
    let log = logger::new(verbose);
    let platform = platform::detect(log)?;
    let temp_folder = tempfile::tempdir().expect("cannot create temp dir");
    let yard = yard::load_or_create(temp_folder.path())?;
    for app in apps {
        let latest_version = app.latest_installable_version(log)?;
        for install_method in app.install_methods() {
            println!("{}", install_method.to_string().bold());
            install::install(&install_method, &latest_version, platform, &yard, log)?;
        }
    }
    Ok(ExitCode::SUCCESS)
}
