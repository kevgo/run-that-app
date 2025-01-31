use crate::commands::{self, available, run, test, update, versions};

/// the main commands that run-this-app can execute
#[derive(Debug, PartialEq)]
pub(crate) enum Command<'a> {
  AppsLong,
  AppsShort,
  Available(available::Args<'a>),
  RunApp(run::Args<'a>),
  DisplayHelp,
  Setup,
  Test(test::Args<'a>),
  Which(commands::which::Args<'a>),
  Update(update::Args),
  Version,
  Versions(versions::Args<'a>),
}
