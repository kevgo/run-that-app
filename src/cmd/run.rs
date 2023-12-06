use crate::apps;
use crate::cli::RequestedApp;
use crate::config;
use crate::error::UserError;
use crate::filesystem::find_global_install;
use crate::platform;
use crate::subshell;
use crate::yard;
use crate::yard::Executable;
use crate::Output;
use crate::Result;
use std::process::ExitCode;

pub fn run(requested_app: RequestedApp, args: Vec<String>, include_path: bool, optional: bool, output: &dyn Output) -> Result<ExitCode> {
    if let Some(executable) = load_or_install(requested_app, include_path, output)? {
        Ok(subshell::execute(executable, args)?)
    } else if optional {
        Ok(ExitCode::SUCCESS)
    } else {
        Err(UserError::UnsupportedPlatform)
    }
}

pub fn load_or_install(mut requested_app: RequestedApp, include_path: bool, output: &dyn Output) -> Result<Option<Executable>> {
    if requested_app.version.is_empty() {
        let config = config::load()?;
        let Some(configured_app) = config.lookup(&requested_app.name) else {
            return Err(UserError::RunRequestMissingVersion);
        };
        requested_app.version = configured_app.version;
    }
    let apps = &apps::all();
    let app = apps.lookup(&requested_app.name)?;
    let platform = platform::detect(output)?;
    let yard = yard::load_or_create(&yard::production_location()?)?;
    if let Some(executable) = yard.load_app(&requested_app, app.executable_filename(platform)) {
        return Ok(Some(executable));
    };
    if yard.is_not_installable(&requested_app) {
        if include_path {
            if let Some(executable) = find_global_install(app.executable_filename(platform), output) {
                return Ok(Some(executable));
            }
        }
        return Ok(None);
    }
    if let Some(executable) = app.install(&requested_app.version, platform, &yard, output)? {
        return Ok(Some(executable));
    }
    yard.mark_not_installable(&requested_app)?;
    if include_path {
        if let Some(executable) = find_global_install(app.executable_filename(platform), output) {
            return Ok(Some(executable));
        }
    }
    Ok(None)
}
