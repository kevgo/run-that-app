use crate::config::{AppName, RequestedVersions};
use crate::platform;
use crate::Result;
use crate::{apps, output};
use std::process::ExitCode;

use super::run::load_or_install;

pub fn which(app: &AppName, versions: &RequestedVersions, log: Option<String>) -> Result<ExitCode> {
    let binding = apps::all();
    let app = binding.lookup(app)?;
    let output = output::StdErr { category: log };
    let platform = platform::detect(&output)?;
    for version in versions.iter() {
        if let Some(executable) = load_or_install(app, version, platform, &output)? {
            println!("{}", executable.0.to_string_lossy());
            return Ok(ExitCode::SUCCESS);
        }
    }
    Ok(ExitCode::FAILURE)
}
