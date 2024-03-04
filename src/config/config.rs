use super::{AppName, Version};
use crate::config::AppVersion;
use std::fmt::Display;

#[derive(Debug, Default, PartialEq)]
pub struct Config {
    pub apps: Vec<AppVersion>,
}

impl Config {
    pub fn lookup(self, app_name: &AppName) -> Option<Version> {
        self.apps.into_iter().find(|app| app.app == app_name).map(|app_version| app_version.version)
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for AppVersion { app, version } in &self.apps {
            f.write_str(app.as_str());
            if let Some(version) = version {
                f.write_str(" ");
                f.write_str(version.as_str());
            }
        }
        Ok(())
    }
}
