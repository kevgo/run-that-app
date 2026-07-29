use crate::applications::{AppDefinition, ApplicationName};
use crate::configuration::{RequestedVersion, RequestedVersions};
use crate::context::RuntimeContext;
use crate::error::Result;
use crate::executables::{Executable, ExecutableNamePlatform, load_from_path, load_from_yard};

/// Loads the given app at the earliest of the given versions that is installable
/// and returns an `ExecutableCall` that runs the given executable within that app
/// with the given arguments.
pub fn load_app_versions(
  app: &dyn AppDefinition,
  versions: &RequestedVersions,
  executable: &ExecutableNamePlatform,
  app_args: &[String],
  ctx: &RuntimeContext,
) -> Result<LoadAppOutcome> {
  for version in versions {
    match version {
      RequestedVersion::Path(version) => {
        if let Some(executable) = load_from_path(app, executable, version, ctx)? {
          return Ok(LoadAppOutcome::Loaded { executable });
        }
      }
      RequestedVersion::Yard(version) => match load_from_yard(app, version, executable, app_args, ctx)? {
        LoadAppOutcome::Loaded { executable } => return Ok(LoadAppOutcome::Loaded { executable }),
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
  Loaded { executable: Executable },
  /// none of the requested versions of the app are installable
  NotInstallable { app: ApplicationName },
  /// the given version of the app is not installed
  /// and not marked as uninstallable
  /// so it should be installed
  NotInstalled { app: ApplicationName },
}
