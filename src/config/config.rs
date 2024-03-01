use std::fmt::Display;

#[derive(Debug, Default, PartialEq)]
pub struct Config {
    pub apps: Vec<AppVersions>,
}

impl Config {
    pub fn lookup(self, app_name: &str) -> Option<AppVersions> {
        self.apps.into_iter().find(|app| app.name == app_name)
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for AppVersions { name, versions } in &self.apps {
            f.write_fmt(format_args!("{name} {version}\n"))?;
        }
        Ok(())
    }
}
