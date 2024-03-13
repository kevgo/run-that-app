use crate::{apps, logger, platform, Result};
use std::process::ExitCode;

pub fn test(verbose: bool) -> Result<ExitCode> {
    let apps = apps::all();
    let log = logger::new(verbose);
    let platform = platform::detect(log)?;
    for app in apps {}
    Ok(ExitCode::SUCCESS)
}
