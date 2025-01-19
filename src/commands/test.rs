use crate::applications::AnalyzeResult;
use crate::configuration::{self, ApplicationName};
use crate::logging::Event;
use crate::prelude::*;
use crate::run::ExecutablePath;
use crate::yard::Yard;
use crate::{applications, installation, logging, platform};
use colored::Colorize;
use std::io;
use std::process::ExitCode;

pub fn test(args: &mut Args) -> Result<ExitCode> {
  let apps = applications::all();
  let log = logging::new(args.verbose);
  let platform = platform::detect(log)?;
  let temp_folder = tempfile::tempdir().map_err(|err| UserError::CannotCreateTempDir { err: err.to_string() })?;
  let yard = Yard::load_or_create(temp_folder.path())?;
  let config_file = configuration::File::load(&apps)?;
  for app in apps {
    if let Some(start_app_name) = &args.start_at_app {
      if app.name() != start_app_name {
        continue;
      }
      args.start_at_app = None;
    }
    log(Event::IntegrationTestNewApp { app: &app.name() });
    let latest_version = app.latest_installable_version(log)?;
    log(Event::IntegrationTestDeterminedVersion { version: &latest_version });
    for install_method in app.run_method(&latest_version, platform).install_methods() {
      log(Event::IntegrationTestNewInstallMethod {
        app: app.name().as_str(),
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
      let executable_path = install_method.executable_location(app.as_ref(), &latest_version, platform, &yard);
      if !executable_path.exists() {
        println!(
          "executable {} not found, press ENTER after inspecting the yard",
          executable_path.to_string_lossy()
        );
        let mut buffer = String::new();
        if let Err(err) = io::stdin().read_line(&mut buffer) {
          eprintln!("Error: {err}");
        }
        return Ok(ExitCode::FAILURE);
      }
      let executable = ExecutablePath(executable_path.clone());
      match app.analyze_executable(&executable, log)? {
        AnalyzeResult::NotIdentified { output } => {
          println!(
            "executable {} not identified based on this output:\n\"{output}\"\nOUTPUT END",
            executable_path.to_string_lossy()
          );
          return Ok(ExitCode::FAILURE);
        }
        AnalyzeResult::IdentifiedButUnknownVersion => println!("{}", "executable identified".green()),
        AnalyzeResult::IdentifiedWithVersion(executable_version) if executable_version == latest_version => {
          println!("{}", "executable has the correct version".green());
        }
        AnalyzeResult::IdentifiedWithVersion(executable_version) => {
          println!("executable has version {executable_version} but we installed version {latest_version}");
          return Ok(ExitCode::FAILURE);
        }
      }
      let _ = yard.delete_app_folder(&app.name());
    }
  }
  Ok(ExitCode::SUCCESS)
}

#[derive(Debug, PartialEq)]
pub struct Args {
  pub optional: bool,
  pub start_at_app: Option<ApplicationName>,
  pub verbose: bool,
}
