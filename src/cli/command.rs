use crate::cmd::run;
use crate::config::AppVersion;

/// the main commands that run-this-app can execute
#[derive(Debug, PartialEq)]
pub enum Command {
    Available { app: AppVersion, include_path: bool, log: Option<String> },
    RunApp { data: run::Data, log: Option<String> },
    DisplayHelp,
    Setup,
    Which { app: AppVersion, include_path: bool, log: Option<String> },
    Update { log: Option<String> },
    Version,
    Versions { app: String, amount: usize, log: Option<String> },
}
