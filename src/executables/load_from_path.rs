//! Loading a globally installed app using the PATH environment variable.

use crate::applications::{AnalyzeResult, AppDefinition};
use crate::context::RuntimeContext;
use crate::error::Result;
use crate::executables::{ExecutableCall, ExecutableNamePlatform};
use crate::filesystem::find_global_install;
use crate::logging::Event;

// finds the given app in the PATH and verifies it has the correct version
pub fn load_from_path(
  app_to_install: &dyn AppDefinition,
  executable_name: &ExecutableNamePlatform,
  range: &semver::VersionReq,
  app_args: &[String],
  ctx: &RuntimeContext,
) -> Result<Option<ExecutableCall>> {
  let Some(executable) = find_global_install(executable_name, ctx.log) else {
    return Ok(None);
  };
  match app_to_install.analyze_executable(&executable, ctx.log)? {
    AnalyzeResult::NotIdentified { output: _ } => {
      (ctx.log)(Event::GlobalInstallNotIdentified);
      Ok(None)
    }
    AnalyzeResult::IdentifiedButUnknownVersion if range.to_string() == "*" => {
      (ctx.log)(Event::GlobalInstallMatchingVersion { range, version: None });
      Ok(Some(ExecutableCall {
        executable,
        args: app_args.to_vec(),
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
      Ok(Some(ExecutableCall {
        executable,
        args: app_args.to_vec(),
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
