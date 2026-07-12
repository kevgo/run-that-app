//! Loading applications from the yard.

use crate::Version;
use crate::applications::AppDefinition;
use crate::context::RuntimeContext;
use crate::error::Result;
use crate::executables::{ExecutableArgs, ExecutableCall, ExecutableNamePlatform, LoadAppOutcome};

/// Loads the given app at the given version
/// and returns a callable that executes it.
pub fn load_from_yard(
  app: &dyn AppDefinition,
  version: &Version,
  executable: &ExecutableNamePlatform,
  args: &ExecutableArgs,
  ctx: &RuntimeContext,
) -> Result<LoadAppOutcome> {
  ctx.yard.with_lock(&app.name(), version, ctx, || {
    // try to load the app from the yard
    if let Some((executable, bin_folder)) = ctx.yard.load_executable(app, executable, version, ctx) {
      let app_folder = ctx.yard.app_folder(&app.name(), version);
      return Ok(LoadAppOutcome::Loaded {
        executable_call: ExecutableCall {
          executable,
          args: args.locate(&app.name(), version, &app_folder, &bin_folder)?,
        },
      });
    }
    // here the app is not installed --> check if it is marked as uninstallable
    if ctx.yard.is_not_installable(&app.name(), version) {
      return Ok(LoadAppOutcome::NotInstallable { app: app.name() });
    }
    // app not installed and installable
    Ok(LoadAppOutcome::NotInstalled { app: app.name() })
  })
}
