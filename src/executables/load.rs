use crate::Version;
use crate::applications::{AnalyzeResult, AppDefinition};
use crate::configuration::{RequestedVersion, RequestedVersions};
use crate::context::RuntimeContext;
use crate::error::Result;
use crate::executables::{ExecutableArgs, ExecutableCall, ExecutableCallDefinition};
use crate::filesystem::find_global_install;
use crate::logging::Event;

/// Loads the given app at the first given version.
/// Only attempts to load subsequently versions
/// if the first version is marked as not installable
/// or optional.
///
/// Returns either the loaded app
/// or information which app version needs to be installed.
pub fn load_app_versions<'a>(
  app_definition: &dyn AppDefinition,
  requested_versions: &'a RequestedVersions,
  executable_args: ExecutableArgs,
  ctx: &RuntimeContext,
) -> Result<LoadAppVersionsOutcome<'a>> {
  for requested_version in requested_versions {
    match requested_version {
      RequestedVersion::Path(version) => {
        if let Some(executable_call_def) = load_from_path(app_definition, version, executable_args.clone(), ctx)?
          && let Some(app_folder) = executable_call_def.executable.clone().as_path().parent()
          && let Some(executable_call) = executable_call_def.into_executable_call(app_folder)
        {
          return Ok(LoadAppVersionsOutcome::Loaded { executable_call });
        } else {
          // the app is not globally installed --> don't install it globally, try the next version
          continue;
        }
      }
      RequestedVersion::Yard(version) => match load_from_yard(app_definition, version, executable_args.clone(), ctx)? {
        LoadAppVersionOutcome::Loaded { executable_call } => return Ok(LoadAppVersionsOutcome::Loaded { executable_call }),
        LoadAppVersionOutcome::NotInstallable => continue,
        LoadAppVersionOutcome::NotInstalled => return Ok(LoadAppVersionsOutcome::MustInstall { version }),
      },
    }
  }
  Ok(LoadAppVersionsOutcome::NotInstallable)
}

pub enum LoadAppVersionsOutcome<'a> {
  /// the app was loaded successfully, here is the executable to call it
  Loaded { executable_call: ExecutableCall },
  /// all requested versions of the app are not installable
  NotInstallable,
  /// the given version of the app is not installed
  /// and not marked as installable
  /// so it should be installed
  MustInstall { version: &'a Version },
}

/// Loads the given app at the given version
/// and returns a callable that executes it.
/// If the app runs via a carrier app,
/// installs the carrier app as well.
fn load_from_yard<'a>(
  app_definition: &dyn AppDefinition,
  version: &Version,
  executable_args: ExecutableArgs,
  ctx: &RuntimeContext,
) -> Result<LoadAppVersionOutcome> {
  // load or install the app
  ctx.yard.with_lock(&app_definition.name(), version, ctx, || {
    // try to load the app
    if let Some((executable, bin_folder)) = ctx.yard.load_executable(app_definition, &app_definition.executable_filename(), version, ctx) {
      let app_folder = ctx.yard.app_folder(&app_definition.name(), version);
      let args = executable_args.locate(&app_definition.name(), version, &app_folder, &bin_folder)?;
      return Ok(LoadAppVersionOutcome::Loaded {
        executable_call: ExecutableCall { executable, args },
      });
    }
    // app not installed --> check if uninstallable
    if ctx.yard.is_not_installable(&app_definition.name(), version) {
      return Ok(LoadAppVersionOutcome::NotInstallable);
    }
    // app not installed and installable
    Ok(LoadAppVersionOutcome::NotInstalled)
  })
}

enum LoadAppVersionOutcome {
  /// the app was loaded successfully, here is the executable to call it
  Loaded { executable_call: ExecutableCall },
  /// the app is marked as not installable
  NotInstallable,
  /// the app is not installed
  NotInstalled,
}

// finds the given app in the PATH and verifies it has the correct version
fn load_from_path(
  app_to_install: &dyn AppDefinition,
  range: &semver::VersionReq,
  executable_args: ExecutableArgs,
  ctx: &RuntimeContext,
) -> Result<Option<ExecutableCallDefinition>> {
  let executable_filename = app_to_install.executable_filename().platform_path(ctx.platform.os);
  let Some(executable) = find_global_install(&executable_filename, ctx.log) else {
    (ctx.log)(Event::GlobalInstallNotFound);
    return Ok(None);
  };
  match app_to_install.analyze_executable(&executable, ctx.log)? {
    AnalyzeResult::NotIdentified { output: _ } => {
      (ctx.log)(Event::GlobalInstallNotIdentified);
      Ok(None)
    }
    AnalyzeResult::IdentifiedButUnknownVersion if range.to_string() == "*" => {
      (ctx.log)(Event::GlobalInstallMatchingVersion { range, version: None });
      Ok(Some(ExecutableCallDefinition {
        executable,
        args: executable_args,
      }))
    }
    AnalyzeResult::IdentifiedButUnknownVersion => {
      (ctx.log)(Event::GlobalInstallMismatchingVersion { range, version: None });
      Ok(None)
    }
    AnalyzeResult::IdentifiedWithVersion(version) if range.matches(&version.semver()?) => {
      (ctx.log)(Event::GlobalInstallMatchingVersion {
        range,
        version: Some(&version),
      });
      Ok(Some(ExecutableCallDefinition {
        executable,
        args: executable_args,
      }))
    }
    AnalyzeResult::IdentifiedWithVersion(version) => {
      (ctx.log)(Event::GlobalInstallMismatchingVersion {
        range,
        version: Some(&version),
      });
      Ok(None)
    }
  }
}
