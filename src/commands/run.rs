use crate::applications::{ApplicationName, Apps};
use crate::error::{Result, UserError};
use crate::{GetCmdArgs, Version, get_cmd, subshell};
use std::path::PathBuf;
use std::process::ExitCode;

pub fn run(args: RunArgs, apps: &Apps) -> Result<ExitCode> {
  let app_to_run = apps.lookup(&args.app_name)?;
  let include_apps = apps.lookup_many(&args.include_apps)?;
  let get_cmd_args = GetCmdArgs {
    version: args.version,
    app_args: args.app_args,
    from_source: args.from_source,
    include_apps,
    optional: args.optional,
    verbose: args.verbose,
  };
  let Some(cmd_info) = get_cmd(app_to_run, get_cmd_args, apps)? else {
    if args.optional {
      return Ok(ExitCode::SUCCESS);
    }
    return Err(UserError::UnsupportedPlatform);
  };
  let cwd = args.cwd.as_deref();
  if args.error_on_output {
    subshell::detect_output(cmd_info, cwd)
  } else {
    subshell::stream_output(cmd_info, cwd)
  }
}

/// data needed to run an executable
#[derive(Debug, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct RunArgs {
  /// name of the app to execute
  pub app_name: ApplicationName,

  /// possible versions of the app to execute
  pub version: Option<Version>,

  /// arguments to call the app with
  #[allow(clippy::struct_field_names)]
  pub app_args: Vec<String>,

  /// if true, any output produced by the app is equivalent to an exit code > 0
  pub error_on_output: bool,

  /// if true, install only from source
  pub from_source: bool,

  /// other applications to include into the PATH
  pub include_apps: Vec<ApplicationName>,

  /// whether it's okay to not run the app if it cannot be installed
  pub optional: bool,

  pub verbose: bool,

  /// optional working directory in which to execute the app
  pub cwd: Option<PathBuf>,
}
