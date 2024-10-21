use crate::cmd::{available, run};
use crate::config::{AppName, Version};

/// the main commands that run-this-app can execute
#[derive(Debug, PartialEq)]
pub enum Command {
  AppsLong,
  AppsShort,
  Available(available::Args),
  RunApp(run::Args),
  DisplayHelp,
  Setup,
  Test { app: Option<AppName>, verbose: bool },
  Which { app: AppName, version: Option<Version>, verbose: bool },
  Update { verbose: bool },
  Version,
  Versions { app: AppName, amount: usize, verbose: bool },
}
