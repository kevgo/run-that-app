use crate::apps;
use crate::config;
use crate::config::AppName;
use crate::config::Version;
use crate::error::UserError;
use crate::filesystem::find_global_install;
use crate::platform;
use crate::subshell;
use crate::subshell::Executable;
use crate::yard;
use crate::Output;
use crate::Result;
use std::process::ExitCode;

pub fn run(data: Data, output: &dyn Output) -> Result<ExitCode> {
    if let Some(executable) = load_or_install(&data.app, data.version, data.include_path, output)? {
        if data.error_on_output {
            Ok(subshell::stream(&executable, &data.app_args)?)
        } else {
            Ok(subshell::run(&executable, &data.app_args)?)
        }
    } else if data.optional {
        Ok(ExitCode::SUCCESS)
    } else {
        Err(UserError::UnsupportedPlatform)
    }
}

#[derive(Debug, PartialEq)]
/// data needed to run an executable
pub struct Data {
    /// name and version of the app to execute
    pub app: AppName,

    pub version: Version,

    /// arguments to call the app with
    pub app_args: Vec<String>,

    /// if true, any output produced by the app is equivalent to an exit code > 0
    pub error_on_output: bool,

    /// whether to include apps in the PATH
    pub include_path: bool,

    /// whether it's okay to not run the app if it cannot be installed
    pub optional: bool,
}

pub fn load_or_install(app_name: &AppName, mut version: Version, include_path: bool, output: &dyn Output) -> Result<Option<Executable>> {
    if version.is_none() {
        let config = config::load()?;
        match config.lookup(app_name) {
            Some(configured_version) => version = configured_version.version,
            None => return Err(UserError::RunRequestMissingVersion),
        }
    }
    let apps = apps::all();
    let app = apps.lookup(app_name)?;
    let platform = platform::detect(output)?;
    let yard = yard::load_or_create(&yard::production_location()?)?;
    if let Some(executable) = app.load(&version, platform, &yard) {
        return Ok(Some(executable));
    };
    if yard.is_not_installable(app_name, &version) {
        if include_path {
            if let Some(executable) = find_global_install(app.executable_filename(platform), output) {
                return Ok(Some(executable));
            }
        }
        return Ok(None);
    }
    if let Some(executable) = app.install(&version, platform, &yard, output)? {
        return Ok(Some(executable));
    }
    yard.mark_not_installable(app_name, &version)?;
    if include_path {
        if let Some(executable) = find_global_install(app.executable_filename(platform), output) {
            return Ok(Some(executable));
        }
    }
    Ok(None)
}
