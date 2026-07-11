//! Loading applications from the yard.

use crate::Version;
use crate::applications::AppDefinition;
use crate::context::RuntimeContext;
use crate::error::Result;
use crate::executables::{ExecutableArgs, ExecutableCall, ExecutableNamePlatform};

/// Loads the given app at the given version
/// and returns a callable that executes it.
pub fn load_from_yard(
  app_definition: &dyn AppDefinition,
  version: &Version,
  executable: &ExecutableNamePlatform,
  executable_args: &ExecutableArgs,
  ctx: &RuntimeContext,
) -> Result<LoadFromYardOutcome> {
  // load or install the app
  ctx.yard.with_lock(&app_definition.name(), version, ctx, || {
    // try to load the app
    if let Some((executable, bin_folder)) = ctx.yard.load_executable(app_definition, executable, version, ctx) {
      let app_folder = ctx.yard.app_folder(&app_definition.name(), version);
      let args = executable_args.locate(&app_definition.name(), version, &app_folder, &bin_folder)?;
      return Ok(LoadFromYardOutcome::Loaded {
        executable_call: ExecutableCall { executable, args },
      });
    }
    // app not installed --> check if uninstallable
    if ctx.yard.is_not_installable(&app_definition.name(), version) {
      return Ok(LoadFromYardOutcome::NotInstallable);
    }
    // app not installed and installable
    Ok(LoadFromYardOutcome::NotInstalled)
  })
}

pub enum LoadFromYardOutcome {
  /// the app was loaded successfully, here is the executable to call it
  Loaded { executable_call: ExecutableCall },
  /// the app is marked as not installable
  NotInstallable,
  /// the app is not installed
  NotInstalled,
}
