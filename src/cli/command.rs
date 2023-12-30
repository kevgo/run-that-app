use super::RequestedApp;

/// the main commands that run-this-app can execute
#[derive(Debug, PartialEq)]
pub enum Command {
    Available {
        app: RequestedApp,
        include_path: bool,
        log: Option<String>,
    },
    RunApp {
        app: RequestedApp,
        args: Vec<String>,
        include_path: bool,
        optional: bool,
        log: Option<String>,
    },
    DisplayHelp,
    Setup,
    Which {
        app: RequestedApp,
        include_path: bool,
        log: Option<String>,
    },
    Update {
        log: Option<String>,
    },
    Version,
    Versions {
        app: String,
        amount: usize,
        log: Option<String>,
    },
}
