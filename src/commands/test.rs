use crate::applications::{AnalyzeResult, ApplicationName, Apps};
use crate::executables::Executable;
use crate::logging::Event;
use crate::prelude::*;
use crate::yard::Yard;
use crate::{applications, configuration, installation, logging, platform};
use colored::Colorize;
use std::io;
use std::process::ExitCode;

pub(crate) fn test(args: &mut Args) -> Result<ExitCode> {
  let apps = applications::all();
  find_duplicate_app_names(&apps)?;
  let log = logging::new(args.verbose);
  let platform = platform::detect(log)?;
  let temp_folder = tempfile::tempdir().map_err(|err| UserError::CannotCreateTempDir { err: err.to_string() })?;
  let yard = Yard::load_or_create(temp_folder.path())?;
  let config_file = configuration::File::load(&apps)?;
  for app in apps {
    if let Some(start_app_name) = &args.start_at_app {
      if &app.app_name() != start_app_name {
        continue;
      }
      args.start_at_app = None;
    }
    log(Event::IntegrationTestNewApp { app: app.name() });
    let latest_version = app.latest_installable_version(log)?;
    log(Event::IntegrationTestDeterminedVersion { version: &latest_version });
    for install_method in app.run_method(&latest_version, platform).install_methods() {
      log(Event::IntegrationTestNewInstallMethod {
        app: app.name(),
        method: &install_method,
        version: &latest_version,
      });
      if !installation::install(
        app.as_ref(),
        &install_method,
        &latest_version,
        platform,
        args.optional,
        &yard,
        &config_file,
        log,
      )?
      .success()
      {
        continue;
      }
      let app_folder = yard.app_folder(&app.app_name(), &latest_version);
      let executable_paths = install_method.executable_paths(&app_folder, &app.executable_filename().platform_path(platform.os));
      let mut executable_found = true;
      for executable_path in executable_paths {
        if !executable_path.exists() {
          continue;
        }
        executable_found = true;
        let executable = Executable::from(executable_path);
        match app.analyze_executable(&executable, log)? {
          AnalyzeResult::NotIdentified { output } => {
            println!("executable {executable} not identified based on this output:\n\"{output}\"\nOUTPUT END");
            return Ok(ExitCode::FAILURE);
          }
          AnalyzeResult::IdentifiedButUnknownVersion => {
            println!("{}", "executable identified".green());
          }
          AnalyzeResult::IdentifiedWithVersion(executable_version) if executable_version == latest_version => {
            println!("{}", "executable has the correct version".green());
          }
          AnalyzeResult::IdentifiedWithVersion(executable_version) => {
            println!("executable has version {executable_version} but we installed version {latest_version}");
            return Ok(ExitCode::FAILURE);
          }
        }
      }
      if !executable_found {
        println!("executable for {app} not found, press ENTER after inspecting the yard");
        let mut buffer = String::new();
        if let Err(err) = io::stdin().read_line(&mut buffer) {
          eprintln!("Error: {err}");
        }
        return Ok(ExitCode::FAILURE);
      }
      let _ = yard.delete_app_folder(&app.app_name());
    }
  }
  Ok(ExitCode::SUCCESS)
}

#[derive(Debug, PartialEq)]
pub(crate) struct Args {
  pub(crate) optional: bool,
  pub(crate) start_at_app: Option<ApplicationName>,
  pub(crate) verbose: bool,
}

fn find_duplicate_app_names(apps: &Apps) -> Result<()> {
  let mut names: Vec<&'static str> = vec![];
  for app in apps {
    let app_name = app.name();
    if names.contains(&app_name) {
      return Err(UserError::DuplicateAppName { name: app_name.to_string() });
    }
    names.push(app_name);
  }
  Ok(())
}
