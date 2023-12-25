use crate::apps;
use crate::output::Output;
use crate::Result;
use std::process::ExitCode;

pub fn versions(app_name: &str, output: &dyn Output) -> Result<ExitCode> {
    let apps = &apps::all();
    let app = apps.lookup(app_name)?;
    let versions = app.versions(10, output)?;
    println!("{app_name} is available in these versions:");
    for version in versions {
        println!("- {version}");
    }
    Ok(ExitCode::SUCCESS)
}
