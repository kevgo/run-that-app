use crate::cli::RequestedApp;

#[derive(Debug, Default, PartialEq)]
pub struct Config {
    pub apps: Vec<RequestedApp>,
}

impl Config {
    pub fn lookup(self, app_name: &str) -> Option<RequestedApp> {
        self.apps.into_iter().find(|app| app.name == app_name)
    }
}
