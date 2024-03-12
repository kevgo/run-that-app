use crate::apps::{AnalyzeResult, App};
use crate::config::{AppName, RequestedVersion, RequestedVersions, Version};
use crate::error::UserError;
use crate::filesystem::find_global_install;
use crate::output::Event;
use crate::platform::{self, Platform};
use crate::subshell;
use crate::subshell::Executable;
use crate::Output;
use crate::Result;
use crate::{apps, config};
use crate::{install, yard};
use colored::Colorize;
use std::process::ExitCode;

pub fn run(args: Args) -> Result<ExitCode> {
    let apps = apps::all();
    let app = apps.lookup(&args.app)?;
    let output = Output { verbose: args.verbose };
    let platform = platform::detect(output)?;
    let versions = RequestedVersions::determine(&args.app, args.version, &apps)?;
    for version in versions {
        if let Some(executable) = load_or_install(app, &version, platform, output)? {
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
pub struct Args {
    /// name of the app to execute
    pub app: AppName,

    /// possible versions of the app to execute
    pub version: Option<Version>,

    /// arguments to call the app with
    #[allow(clippy::struct_field_names)]
    pub app_args: Vec<String>,

    /// if true, any output produced by the app is equivalent to an exit code > 0
    pub error_on_output: bool,

    /// whether it's okay to not run the app if it cannot be installed
    pub optional: bool,

    pub verbose: bool,
}

pub fn load_or_install(app: &dyn App, version: &RequestedVersion, platform: Platform, output: Output) -> Result<Option<Executable>> {
    match version {
        RequestedVersion::Path(version) => load_from_path(app, version, platform, output),
        RequestedVersion::Yard(version) => load_or_install_from_yard(app, version, output),
    }
}

// checks if the app is in the PATH and has the correct version
fn load_from_path(app: &dyn App, want_version: &semver::VersionReq, platform: Platform, output: Output) -> Result<Option<Executable>> {
    let Some(executable) = find_global_install(&app.executable_filename(platform), output) else {
        return Ok(None);
    };
    let analyze_result = app.analyze_executable(&executable);
    output.log(Event::GlobalInstallAnalyzed {
        executable: &executable,
        app_name: &app.name(),
        result: &analyze_result,
    });
    match analyze_result {
        AnalyzeResult::NotIdentified => Ok(None),
        AnalyzeResult::IdentifiedButUnknownVersion if want_version.to_string() == "*" => Ok(Some(executable)),
        AnalyzeResult::IdentifiedButUnknownVersion => Ok(None),
        AnalyzeResult::IdentifiedWithVersion(version) if want_version.matches(&version.semver()?) => Ok(Some(executable)),
        AnalyzeResult::IdentifiedWithVersion(version) => Ok(None),
    }
}

fn load_or_install_from_yard(app: &dyn App, version: &Version, output: Output) -> Result<Option<Executable>> {
    let platform = platform::detect(output)?;
    let yard = yard::load_or_create(&yard::production_location()?)?;
    // try to load the app
    if let Some(executable) = install::load(app.install_methods(), version, platform, &yard) {
        return Ok(Some(executable));
    }
    // app not installed --> check if uninstallable
    if yard.is_not_installable(&app.name(), version) {
        return Ok(None);
    }
    // app not installed and installable --> try to install
    if install::install(app.install_methods(), version, platform, output)? {
        return Ok(install::load(app.install_methods(), version, platform, &yard));
    }

    // app could not be installed -> mark as uninstallable
    yard.mark_not_installable(&app.name(), version)?;
    Ok(None)
}
