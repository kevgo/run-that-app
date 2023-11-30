use crate::apps;
use crate::cli::RequestedApp;
use crate::config;
use crate::error::UserError;
use crate::platform;
use crate::yard;
use crate::Output;
use crate::Result;
use std::process::ExitCode;

use super::run::load_or_install;

pub fn show_path(mut requested_app: RequestedApp, include_global: bool, output: &dyn Output) -> Result<ExitCode> {
    if requested_app.version.is_empty() {
        let config = config::load()?;
        let Some(configured_app) = config.lookup(&requested_app.name) else {
            return Err(UserError::RunRequestMissingVersion);
        };
        requested_app.version = configured_app.version;
    }
    let app = apps::lookup(&requested_app.name)?;
    let platform = platform::detect(output)?;
    let prodyard = yard::load_or_create(&yard::production_location()?)?;
    if let Some(executable) = load_or_install(&requested_app, app.as_ref(), platform, include_global, &prodyard, output)? {
        println!("{}", executable.0.to_string_lossy());
    }
    Ok(ExitCode::SUCCESS)
}
