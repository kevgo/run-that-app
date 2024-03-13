use crate::apps;
use crate::config::AppName;
use crate::output;
use crate::Result;
use std::process::ExitCode;

pub fn versions(app_name: &AppName, amount: usize, verbose: bool) -> Result<ExitCode> {
    let apps = &apps::all();
    let app = apps.lookup(app_name)?;
    let output = output::new(verbose);
    let versions = app.installable_versions(amount, output)?;
    println!("{app_name} is available in these versions:");
    for version in versions {
        println!("- {version}");
    }
    Ok(ExitCode::SUCCESS)
}
