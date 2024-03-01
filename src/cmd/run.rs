use crate::apps;
use crate::cli::AppVersion;
use crate::config;
use crate::error::UserError;
use crate::filesystem::find_global_install;
use crate::platform;
use crate::subshell;
use crate::subshell::Executable;
use crate::yard;
use crate::Output;
use crate::Result;
use std::process::ExitCode;

pub fn run(args: Data, output: &dyn Output) -> Result<ExitCode> {
    if let Some(executable) = load_or_install(args.app_version, args.include_path, output)? {
        if args.error_on_output {
            Ok(subshell::stream(&executable, &args.app_args)?)
        } else {
            Ok(subshell::run(&executable, &args.app_args)?)
        }
    } else if args.optional {
        Ok(ExitCode::SUCCESS)
    } else {
        Err(UserError::UnsupportedPlatform)
    }
}

#[derive(Debug, PartialEq)]
/// data needed to run an executable
pub struct Data {
    pub app_version: AppVersion,
    pub app_args: Vec<String>,
    pub error_on_output: bool,
    pub include_path: bool,
    pub optional: bool,
}

pub fn load_or_install(mut app_version: AppVersion, include_path: bool, output: &dyn Output) -> Result<Option<Executable>> {
    if app_version.version.is_empty() {
        let config = config::load()?;
        let Some(configured_app) = config.lookup(&app_version.name) else {
            return Err(UserError::RunRequestMissingVersion);
        };
        app_version.version = configured_app.version;
    }
    let apps = &apps::all();
    let app = apps.lookup(&app_version.name)?;
    let platform = platform::detect(output)?;
    let yard = yard::load_or_create(&yard::production_location()?)?;
    if let Some(executable) = app.load(&app_version.version, platform, &yard) {
        return Ok(Some(executable));
    };
    if yard.is_not_installable(&app_version) {
        if include_path {
            if let Some(executable) = find_global_install(app.executable_filename(platform), output) {
                return Ok(Some(executable));
            }
        }
        return Ok(None);
    }
    if let Some(executable) = app.install(&app_version.version, platform, &yard, output)? {
        return Ok(Some(executable));
    }
    yard.mark_not_installable(&app_version)?;
    if include_path {
        if let Some(executable) = find_global_install(app.executable_filename(platform), output) {
            return Ok(Some(executable));
        }
    }
    Ok(None)
}
