//! Loading applications from the yard.

use crate::Version;
use crate::applications::AppDefinition;
use crate::context::RuntimeContext;
use crate::error::Result;
use crate::executables::{ExecutableCall, ExecutableNamePlatform, LoadAppOutcome};

/// Loads the given app at the given version
/// and returns a callable that executes it.
pub fn load_from_yard(
  app: &dyn AppDefinition,
  version: &Version,
  executable: &ExecutableNamePlatform,
  app_args: &[String],
  ctx: &RuntimeContext,
) -> Result<LoadAppOutcome> {
  println!("1111111111111111111111111111111111111111111111 {app_args:?}");
  ctx.yard.with_lock(&app.name(), version, ctx, || {
    // try to load the app from the yard
    if let Some(executable) = ctx.yard.load_executable(app, executable, version, ctx) {
      return Ok(LoadAppOutcome::Loaded {
        executable_call: ExecutableCall {
          executable,
          args: app_args.to_vec(),
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
