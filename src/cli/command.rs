use crate::commands::{AddArgs, AvailableArgs, InstallArgs, RunArgs, TestArgs, UpdateArgs, VersionsArgs, WhichArgs};

/// the main commands that run-this-app can execute
#[derive(Debug, PartialEq)]
pub enum Command {
  Add(AddArgs),
  AppsLong,
  AppsShort,
  Available(AvailableArgs),
  DisplayHelp,
  Install(InstallArgs),
  InstallAll,
  Reinstall(InstallArgs),
  RunApp(RunArgs),
  Test(TestArgs),
  Update(UpdateArgs),
  Version,
  Versions(VersionsArgs),
  Which(WhichArgs),
}
