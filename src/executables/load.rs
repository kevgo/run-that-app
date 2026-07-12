use crate::applications::{AppDefinition, ApplicationName};
use crate::configuration::{RequestedVersion, RequestedVersions};
use crate::context::RuntimeContext;
use crate::error::Result;
use crate::executables::load_from_yard::LoadFromYardOutcome;
use crate::executables::{ExecutableArgs, ExecutableCall, ExecutableNamePlatform, load_from_path, load_from_yard};

/// Loads the given app at the earliest of the given versions that is installable
/// and returns an `ExecutableCall` that runs the given executable within that app
/// with the given arguments.
pub fn load_app_versions(
  app: &dyn AppDefinition,
  versions: &RequestedVersions,
  executable: &ExecutableNamePlatform,
  args: &ExecutableArgs,
  ctx: &RuntimeContext,
) -> Result<LoadAppVersionsOutcome> {
  for version in versions {
    match version {
      RequestedVersion::Path(version) => {
        if let Some(executable_call_def) = load_from_path(app, executable, version, args.clone(), ctx)?
          && let Some(app_folder) = executable_call_def.executable.clone().as_path().parent()
          && let Some(executable_call) = executable_call_def.into_executable_call(app_folder)
        {
          return Ok(LoadAppVersionsOutcome::Loaded { executable_call });
        }
      }
      RequestedVersion::Yard(version) => match load_from_yard(app, version, executable, args, ctx)? {
        LoadFromYardOutcome::Loaded { executable_call } => return Ok(LoadAppVersionsOutcome::Loaded { executable_call }),
        LoadFromYardOutcome::NotInstallable => {}
        LoadFromYardOutcome::NotInstalled => {
          return Ok(LoadAppVersionsOutcome::NotInstalled { app: app.name() });
        }
      },
    }
  }
  Ok(LoadAppVersionsOutcome::NotInstallable { app: app.name().clone() })
}

pub enum LoadAppVersionsOutcome {
  /// the app was loaded successfully, here is the executable to call it
  Loaded { executable_call: ExecutableCall },
  /// none of the requested versions of the app are installable
  NotInstallable { app: ApplicationName },
  /// the given version of the app is not installed
  /// and not marked as uninstallable
  /// so it should be installed
  NotInstalled { app: ApplicationName },
}
