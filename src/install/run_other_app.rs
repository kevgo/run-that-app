use crate::apps;
use crate::apps::App;
use crate::cmd::run::load_or_install;
use crate::config::{RequestedVersion, Version};
use crate::logger::Log;
use crate::platform::Platform;
use crate::prelude::*;
use crate::yard::Yard;
use std::process::Command;

/// defines the information needed for apps whose executable is shipped as part of another app
pub trait RunOtherApp: App {
  /// the application to run
  fn app_to_run(&self, version: &Version, platform: Platform) -> Box<dyn App>;

  /// the call of the other app that runs this app
  fn call_args(&self) -> Vec<&str>;
}

// pub fn run_other_app(app: &dyn RunOtherApp, version: &Version, platform: Platform, yard: &Yard, log: Log) -> Result<ExitCode> {
//   let app_to_run = app.app_to_run(version, platform);
//   let all_apps = apps::all();
//   let other_app = all_apps.lookup(&app_to_run.name())?;
//   // Note: we know it must be the Yard variant here.
//   // At this point we are installing the app.
//   // Only Yard variants get installed. The Path variant doesn't get installed.
//   let executable = load_or_install(other_app, &RequestedVersion::Yard(version.to_owned()), platform, yard, log)?;
//   let Some(executable) = executable else {
//     return Err(UserError::UnsupportedPlatform);
//   };
//   // run the executable here
//   let mut cmd = Command::new(executable);
//   let call_args = app.call_args();
//   cmd.args(call_args);
//   let status = cmd.status().unwrap();
//   Ok(status.code())
// }
