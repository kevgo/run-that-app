use crate::config::{AppName, RequestedVersion, RequestedVersions, Version};
use crate::error::UserError;
use crate::filesystem::find_global_install;
use crate::platform;
use crate::subshell;
use crate::subshell::Executable;
use crate::yard;
use crate::Output;
use crate::Result;
use crate::{apps, config};
use colored::Colorize;
use std::process::ExitCode;

pub fn run(args: &Args) -> Result<ExitCode> {
    for version in args.versions.iter() {
        if let Some(executable) = load_or_install(&args.app, version, args.output)? {
            if args.error_on_output {
                return subshell::execute_check_output(&executable, &args.app_args);
            }
            return subshell::execute_stream_output(&executable, &args.app_args);
        }
    }
    if args.optional {
        Ok(ExitCode::SUCCESS)
    } else {
        Err(UserError::UnsupportedPlatform)
    }
}

/// data needed to run an executable
pub struct Args<'a> {
    /// name of the app to execute
    pub app: AppName,

    /// possible versions of the app to execute
    pub versions: RequestedVersions,

    /// arguments to call the app with
    #[allow(clippy::struct_field_names)]
    pub app_args: Vec<String>,

    /// if true, any output produced by the app is equivalent to an exit code > 0
    pub error_on_output: bool,

    /// whether it's okay to not run the app if it cannot be installed
    pub optional: bool,

    pub output: &'a dyn Output,
}

pub fn load_or_install(app_name: &AppName, version: &RequestedVersion, output: &dyn Output) -> Result<Option<Executable>> {
    match version {
        RequestedVersion::Path(version) => load_from_path(app_name, &parse_semver_req(version)?, output),
        RequestedVersion::Yard(version) => load_or_install_from_yard(app_name, version, output),
    }
}

// checks if the app is in the PATH and has the correct version
fn load_from_path(app_name: &AppName, want_version: &semver::VersionReq, output: &dyn Output) -> Result<Option<Executable>> {
    let apps = apps::all();
    let app = apps.lookup(app_name)?;
    let platform = platform::detect(output)?;
    let Some(executable) = find_global_install(app.executable_filename(platform), output) else {
        return Ok(None);
    };
    match app.identify_executable(&executable) {
        apps::IdentifyResult::NotIdentified => {
            output.println(&format!(
                "found {} but it doesn't seem an {} executable",
                executable.as_str().cyan().bold(),
                app_name.as_str().cyan().bold()
            ));
            Ok(None)
        }
        apps::IdentifyResult::IdentifiedButUnknownVersion if want_version.to_string() == "*" => Ok(Some(executable)),
        apps::IdentifyResult::IdentifiedButUnknownVersion => {
            output.println(&format!(
                "{} is an {} executable but I'm unable to determine its version.",
                executable.as_str().cyan().bold(),
                app_name.as_str().cyan().bold(),
            ));
            Ok(None)
        }
        apps::IdentifyResult::IdentifiedWithVersion(version) if want_version.matches(&version.semver()?) => Ok(Some(executable)),
        apps::IdentifyResult::IdentifiedWithVersion(version) => {
            output.println(&format!(
                "\n{} is version {} but {} requires {}",
                executable.as_str().green().bold(),
                version.as_str().cyan().bold(),
                config::FILE_NAME.green().bold(),
                want_version.to_string().cyan().bold(),
            ));
            Ok(None)
        }
    }
}

fn load_or_install_from_yard(app_name: &AppName, version: &Version, output: &dyn Output) -> Result<Option<Executable>> {
    let apps = apps::all();
    let app = apps.lookup(app_name)?;
    let platform = platform::detect(output)?;
    let yard = yard::load_or_create(&yard::production_location()?)?;
    if let Some(executable) = app.load(version, platform, &yard) {
        return Ok(Some(executable));
    };
    if yard.is_not_installable(app_name, version) {
        return Ok(None);
    }
    if let Some(executable) = app.install(version, platform, &yard, output)? {
        return Ok(Some(executable));
    }
    yard.mark_not_installable(app_name, version)?;
    Ok(None)
}

fn parse_semver_req(text: &str) -> Result<semver::VersionReq> {
    semver::VersionReq::parse(text).map_err(|err| UserError::CannotParseSemverRange {
        expression: text.to_string(),
        reason: err.to_string(),
    })
}
