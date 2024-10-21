use crate::cmd::{available, run, test};
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
  Test(test::Args),
  Which { app: AppName, version: Option<Version>, verbose: bool },
  Update { verbose: bool },
  Version,
  Versions { app: AppName, amount: usize, verbose: bool },
}
