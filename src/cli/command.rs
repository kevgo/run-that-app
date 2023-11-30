use super::RequestedApp;

/// the main commands that run-this-app can execute
#[derive(Debug, PartialEq)]
pub enum Command {
    Available {
        app: RequestedApp,
        include_global: bool,
        log: Option<String>,
    },
    RunApp {
        app: RequestedApp,
        args: Vec<String>,
        include_global: bool,
        optional: bool,
        log: Option<String>,
    },
    DisplayHelp,
    ShowPath {
        app: RequestedApp,
        include_global: bool,
        log: Option<String>,
    },
    DisplayVersion,
}
