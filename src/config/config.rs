use crate::cli::RequestedApp;

#[derive(Debug, Default, PartialEq)]
pub struct Config(pub Vec<RequestedApp>);

impl Config {
    pub fn lookup(self, app_name: &str) -> Option<RequestedApp> {
        self.0.into_iter().find(|app| app.name == app_name)
    }
}
