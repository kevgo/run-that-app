use crate::applications::{AppDefinition, ApplicationName};
use crate::configuration::{RequestedVersion, RequestedVersions};
use crate::context::RuntimeContext;
use crate::error::Result;
use crate::executables::{ExecutableCall, ExecutableNamePlatform, load_from_path, load_from_yard};

/// Loads the given app at the earliest of the given versions that is installable
/// and returns an `ExecutableCall` that runs the given executable within that app
/// with the given arguments.
pub fn load_app_versions(
  app: &dyn AppDefinition,
  versions: &RequestedVersions,
  executable: &ExecutableNamePlatform,
  app_args: Vec<String>,
  ctx: &RuntimeContext,
) -> Result<LoadAppOutcome> {
  for version in versions {
    match version {
      RequestedVersion::Path(version) => {
        if let Some(executable_call) = load_from_path(app, executable, version, app_args.clone(), ctx)? {
          return Ok(LoadAppOutcome::Loaded { executable_call });
        }
      }
      RequestedVersion::Yard(version) => match load_from_yard(app, version, executable, app_args.clone(), ctx)? {
        LoadAppOutcome::Loaded { executable_call } => return Ok(LoadAppOutcome::Loaded { executable_call }),
        LoadAppOutcome::NotInstallable { app: _ } => {}
        LoadAppOutcome::NotInstalled { app } => {
          return Ok(LoadAppOutcome::NotInstalled { app });
        }
      },
    }
  }
  Ok(LoadAppOutcome::NotInstallable { app: app.name().clone() })
}

pub enum LoadAppOutcome {
  /// the app was loaded successfully, here is the executable to call it
  Loaded { executable_call: ExecutableCall },
  /// none of the requested versions of the app are installable
  NotInstallable { app: ApplicationName },
  /// the given version of the app is not installed
  /// and not marked as uninstallable
  /// so it should be installed
  NotInstalled { app: ApplicationName },
}
