use super::RequestedApp;

/// the main commands that run-this-app can execute
#[derive(Debug, PartialEq)]
pub enum Command {
    RunApp { app: RequestedApp },
    DisplayHelp,
    DisplayVersion,
}
