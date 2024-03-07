use crate::config::AppName;
use crate::Result;
use crate::{apps, output};
use std::process::ExitCode;

pub fn versions(app_name: &AppName, amount: usize, log: Option<String>) -> Result<ExitCode> {
    let apps = &apps::all();
    let app = apps.lookup(app_name)?;
    let output = output::StdErr { category: log };
    let versions = app.installable_versions(amount, &output)?;
    println!("{app_name} is available in these versions:");
    for version in versions {
        println!("- {version}");
    }
    Ok(ExitCode::SUCCESS)
}
