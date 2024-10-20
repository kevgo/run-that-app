use crate::config::{AppName, Version};

/// the main commands that run-this-app can execute
#[derive(Debug, PartialEq)]
pub enum Command {
  AppsLong,
  AppsShort,
  Available {
    app: AppName,
    version: Option<Version>,
    verbose: bool,
  },
  RunApp {
    app: AppName,
    version: Option<Version>,
    app_args: Vec<String>,
    error_on_output: bool,
    optional: bool,
    verbose: bool,
  },
  DisplayHelp,
  Setup,
  Test {
    app: Option<AppName>,
    verbose: bool,
  },
  Which {
    app: AppName,
    version: Option<Version>,
    verbose: bool,
  },
  Update {
    verbose: bool,
  },
  Version,
  Versions {
    app: AppName,
    amount: usize,
    verbose: bool,
  },
}
