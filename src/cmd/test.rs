use crate::apps::AnalyzeResult;
use crate::config::{self, AppName};
use crate::logger::Event;
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{apps, install, logger, platform, yard};
use colored::Colorize;
use std::io;
use std::process::ExitCode;

pub fn test(args: &mut Args) -> Result<ExitCode> {
  let apps = apps::all();
  let log = logger::new(args.verbose);
  let platform = platform::detect(log)?;
  let temp_folder = tempfile::tempdir().map_err(|err| UserError::CannotCreateTempDir { err: err.to_string() })?;
  let yard = yard::load_or_create(temp_folder.path())?;
  let config_file = config::File::load(&apps)?;
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
    for install_method in app.install_methods() {
      log(Event::IntegrationTestNewInstallMethod {
        version: &latest_version,
        method: &install_method,
      });
      if !install::install(&install_method, &latest_version, platform, &yard, &config_file, log)?.success() {
        continue;
      }
      let executable_location = install_method.executable_location(&latest_version, platform);
      let executable_path = yard.app_folder(&install_method.yard_app(), &latest_version).join(executable_location);
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
      let executable = Executable(executable_path.clone());
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
    }
  }
  Ok(ExitCode::SUCCESS)
}

#[derive(Debug, PartialEq)]
pub struct Args {
  pub start_at_app: Option<AppName>,
  pub verbose: bool,
}
