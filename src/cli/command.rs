use crate::cmd::run;
use crate::config::{AppName, Version};

/// the main commands that run-this-app can execute
#[derive(Debug, PartialEq)]
pub enum Command {
    Available {
        app: AppName,
        version: Version,
        include_path: bool,
        log: Option<String>,
    },
    RunApp {
        data: run::Data,
        log: Option<String>,
    },
    DisplayHelp,
    Setup,
    Which {
        app: AppName,
        version: Version,
        include_path: bool,
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
