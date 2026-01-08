use crate::commands::{add, available, install, run, test, update, versions, which};

/// the main commands that run-this-app can execute
#[derive(Debug, PartialEq)]
pub(crate) enum Command {
  Add(add::Args),
  AppsLong,
  AppsShort,
  Available(available::Args),
  DisplayHelp,
  Install(install::Args),
  Reinstall(install::Args),
  RunApp(run::Args),
  Test(test::Args),
  Update(update::Args),
  Version,
  Versions(versions::Args),
  Which(which::Args),
}
