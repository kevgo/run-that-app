use super::AppName;
use crate::config::AppVersion;
use std::fmt::Display;

use super::AppVersions;

#[derive(Debug, Default, PartialEq)]
pub struct Config {
    pub apps: Vec<AppVersions>,
}

impl Config {
    pub fn lookup(self, app_name: &AppName) -> Option<AppVersions> {
        self.apps.into_iter().find(|app| app.app == app_name)
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for AppVersion { app, version } in &self.apps {
            f.write_fmt(format_args!("{name} {}\n", versions.join(", ")))?;
        }
        Ok(())
    }
}
