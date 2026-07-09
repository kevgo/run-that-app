use crate::Version;
use crate::applications::AppDefinition;
use crate::configuration::{RequestedVersion, RequestedVersions};
use crate::context::RuntimeContext;
use crate::error::Result;
use crate::executables::load_from_yard::LoadFromYardOutcome;
use crate::executables::{ExecutableArgs, ExecutableCall, load_from_path, load_from_yard};

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
        LoadFromYardOutcome::Loaded { executable_call } => return Ok(LoadAppVersionsOutcome::Loaded { executable_call }),
        LoadFromYardOutcome::NotInstallable => continue,
        LoadFromYardOutcome::NotInstalled => return Ok(LoadAppVersionsOutcome::MustInstall { version }),
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
