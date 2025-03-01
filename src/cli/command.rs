use crate::commands::{add, available, run, test, update, versions, which};

/// the main commands that run-this-app can execute
#[derive(Debug, PartialEq)]
pub(crate) enum Command {
  Add(add::Args),
  AppsLong,
  AppsShort,
  Available(available::Args),
  RunApp(run::Args),
  DisplayHelp,
  Test(test::Args),
  Which(which::Args),
  Update(update::Args),
  Version,
  Versions(versions::Args),
}
