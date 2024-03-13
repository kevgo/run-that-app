use crate::apps;
use crate::config::AppName;
use crate::logger;
use crate::Result;
use std::process::ExitCode;

pub fn versions(app_name: &AppName, amount: usize, verbose: bool) -> Result<ExitCode> {
    let apps = &apps::all();
    let app = apps.lookup(app_name)?;
    let log = logger::new(verbose);
    let versions = app.installable_versions(amount, log)?;
    println!("{app_name} is available in these versions:");
    for version in versions {
        println!("- {version}");
    }
    Ok(ExitCode::SUCCESS)
}
