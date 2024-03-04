use super::AppName;
use super::AppVersions;
use std::fmt::Display;

#[derive(Debug, Default, PartialEq)]
pub struct Config {
    pub apps: Vec<AppVersions>,
}

impl Config {
    pub fn lookup(self, app_name: &AppName) -> Option<AppVersions> {
        self.apps.into_iter().find(|app| &app.app == app_name)
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for AppVersions { app, versions } in &self.apps {
            f.write_str(app.as_str());
            f.write_str(" ");
            let texts: Vec<&str> = versions.into_iter().map(|version| version.as_str()).collect();
            f.write_str(&texts.join(", "));
        }
        Ok(())
    }
}
