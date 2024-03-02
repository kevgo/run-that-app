use crate::apps;
use crate::config::AppName;
use crate::output::Output;
use crate::Result;
use std::process::ExitCode;

pub fn versions(app_name: &AppName, amount: usize, output: &dyn Output) -> Result<ExitCode> {
    let apps = &apps::all();
    let app = apps.lookup(app_name)?;
    let versions = app.installable_versions(amount, output)?;
    println!("{app_name} is available in these versions:");
    for version in versions {
        println!("- {version}");
    }
    Ok(ExitCode::SUCCESS)
}
