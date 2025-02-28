use crate::applications::ApplicationName;
use crate::commands::{self, available, run, test, update, versions};

/// the main commands that run-this-app can execute
#[derive(Debug, PartialEq)]
pub(crate) enum Command {
  Add(commands::add::Args),
  AppsLong,
  AppsShort,
  Available(available::Args),
  RunApp(run::Args),
  DisplayHelp,
  Test(test::Args),
  Which(commands::which::Args),
  Update(update::Args),
  Version,
  Versions(versions::Args),
}
