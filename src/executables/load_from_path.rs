//! Loading a globally installed app using the PATH environment variable.

use crate::applications::{AnalyzeResult, AppDefinition};
use crate::context::RuntimeContext;
use crate::error::Result;
use crate::executables::{ExecutableArgs, ExecutableCallDefinition};
use crate::filesystem::find_global_install;
use crate::logging::Event;

// finds the given app in the PATH and verifies it has the correct version
pub fn load_from_path(
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
