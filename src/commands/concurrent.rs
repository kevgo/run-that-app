use super::run::{load_or_install_app, load_or_install_apps};
use crate::applications::{ApplicationName, Apps};
use crate::cli::AppVersion;
use crate::configuration::{self, RequestedVersions};
use crate::context::RuntimeContext;
use crate::error::{Result, UserError};
use crate::logging;
use crate::platform;
use crate::yard::Yard;
use crate::{subshell, yard};
use std::process::ExitCode;
use std::sync::{Arc, Mutex};
use std::thread;

pub(crate) fn concurrent(args: Args, apps: &Apps) -> Result<ExitCode> {
  let log = logging::new(args.verbose);
  let platform = platform::detect(log)?;
  let yard = Yard::load_or_create(&yard::production_location()?)?;
  let config_file = configuration::File::load(apps)?;
  let ctx = RuntimeContext {
    platform,
    yard: &yard,
    config_file: &config_file,
    log,
  };

  // Load included apps once for all commands
  let include_app_versions = config_file.lookup_many(args.include_apps);
  let include_apps = load_or_install_apps(include_app_versions, apps, args.optional, args.from_source, &ctx)?;

  // Prepare all applications to run
  let mut tasks = Vec::new();
  for command_str in &args.commands {
    // Split the command string into executable name and arguments
    let parts = shell_words::split(command_str).map_err(|_| UserError::InvalidCommandString(command_str.clone()))?;
    if parts.is_empty() {
      continue; // Skip empty commands
    }
    // Parse app name and version from the first part
    let app_version = AppVersion::new(&parts[0], apps)?;
    let app_name = app_version.app_name;
    let version = app_version.version;
    let app_args: Vec<String> = parts.into_iter().skip(1).collect();

    // Load or install the application
    let app_definition = apps.lookup(&app_name)?;
    let requested_versions = RequestedVersions::determine(&app_name, version.as_ref(), &config_file)?;
    let Some(executable_call) = load_or_install_app(app_definition, requested_versions, args.optional, args.from_source, &ctx)? else {
      if args.optional {
        continue;
      }
      return Err(UserError::UnsupportedPlatform);
    };

    tasks.push((executable_call, app_args));
  }

  // Run all tasks concurrently
  let exit_codes = Arc::new(Mutex::new(Vec::new()));
  let mut handles = Vec::new();

  for (executable_call, app_args) in tasks {
    let exit_codes_clone = Arc::clone(&exit_codes);
    let include_apps_clone = include_apps.clone();
    let handle = thread::spawn(move || {
      let (executable, args) = executable_call.with_args(app_args);
      let result = subshell::stream_output(&executable, &args, &include_apps_clone);

      match result {
        Ok(exit_code) => {
          let mut codes = exit_codes_clone.lock().unwrap();
          codes.push(exit_code);
        }
        Err(err) => {
          // If there's an error, we'll treat it as a failure
          let mut codes = exit_codes_clone.lock().unwrap();
          codes.push(ExitCode::FAILURE);
          eprintln!("Error executing command: {:?}", err);
        }
      }
    });
    handles.push(handle);
  }

  // Wait for all threads to complete
  for handle in handles {
    handle.join().unwrap();
  }

  // Determine the final exit code
  let codes = exit_codes.lock().unwrap();

  // If any command failed, return the first failure code
  for code in codes.iter() {
    if *code != ExitCode::SUCCESS {
      return Ok(*code);
    }
  }

  Ok(ExitCode::SUCCESS)
}

/// data needed to run multiple executables concurrently
#[derive(Debug, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct Args {
  /// commands to execute (each as a full command string)
  pub(crate) commands: Vec<String>,

  /// if true, install only from source
  pub(crate) from_source: bool,

  /// other applications to include into the PATH
  pub(crate) include_apps: Vec<ApplicationName>,

  /// whether it's okay to not run the app if it cannot be installed
  pub(crate) optional: bool,

  pub(crate) verbose: bool,
}
