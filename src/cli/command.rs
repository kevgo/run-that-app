use crate::config::{AppName, Version};

/// the main commands that run-this-app can execute
#[derive(Debug, PartialEq)]
pub enum Command {
    Available {
        app: AppName,
        version: Option<Version>,
        log: Option<String>,
    },
    RunApp {
        app: AppName,
        version: Option<Version>,
        app_args: Vec<String>,
        error_on_output: bool,
        optional: bool,
        log: Option<String>,
    },
    DisplayHelp,
    Setup,
    Which {
        app: AppName,
        version: Option<Version>,
        log: Option<String>,
    },
    Update {
        log: Option<String>,
    },
    Version,
    Versions {
        app: AppName,
        amount: usize,
        log: Option<String>,
    },
}
