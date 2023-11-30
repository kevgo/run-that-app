use crate::apps;
use crate::apps::App;
use crate::cli::RequestedApp;
use crate::config;
use crate::error::UserError;
use crate::filesystem::find_global_install;
use crate::platform;
use crate::platform::Platform;
use crate::subshell;
use crate::yard;
use crate::yard::Executable;
use crate::yard::LoadAppOutcome;
use crate::yard::Yard;
use crate::Output;
use crate::Result;
use std::process::ExitCode;

pub fn run(mut requested_app: RequestedApp, args: Vec<String>, include_global: bool, output: &dyn Output) -> Result<ExitCode> {
  if requested_app.version.is_empty() {
    let config = config::load()?;
    let Some(configured_app) = config.lookup(&requested_app.name) else {
      return Err(UserError::RunRequestMissingVersion);
    };
    requested_app.version = configured_app.version;
  }
  let app = apps::lookup(&requested_app.name)?;
  let platform = platform::detect(output)?;
  let prodyard = yard::load_or_create(&yard::production_location()?)?;
  let load_outcome = load_or_install(&requested_app, app.as_ref(), platform, include_global, &prodyard, output)?;
  match load_outcome {
    LoadAppOutcome::Loaded(executable) => Ok(subshell::execute(executable, args)),
    LoadAppOutcome::NotInstalled if include_global => {
      if let Some(global_app) = find_global_install(app.executable_filename(platform), output) {
        Ok(subshell::execute(global_app, args))
      } else {
        Err(UserError::UnsupportedPlatform)
      }
    }
    LoadAppOutcome::NotInstalled => Err(UserError::UnsupportedPlatform),
    LoadAppOutcome::NotInstallable => Err(UserError::UnsupportedPlatform),
  }
}

fn load_or_install(
  requested_app: &RequestedApp,
  app: &dyn App,
  platform: Platform,
  include_global: bool,
  yard: &Yard,
  output: &dyn Output,
) -> Result<LoadAppOutcome> {
  match yard.load_app(requested_app, app.executable_filename(platform)) {
    LoadAppOutcome::Loaded(executable) => Ok(executable),
    LoadAppOutcome::NotInstalled => todo!(),
    LoadAppOutcome::NotInstallable => todo!(),
  };
  for installation_method in app.installation_methods(&requested_app.version, platform, yard) {
    if let Some(executable) = installation_method.install(output)? {
      return Ok(executable);
    }
  }
  if include_global {
    if let Some(executable) = find_global_install(app.executable_filename(platform), output) {
      return Ok(Executable(executable));
    }
  }
  Err(UserError::UnsupportedPlatform)
}
