use crate::cmd::{self, available, run, test};
use crate::config::AppName;

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
  Which(cmd::which::Args),
  Update { verbose: bool },
  Version,
  Versions { app: AppName, amount: usize, verbose: bool },
}
