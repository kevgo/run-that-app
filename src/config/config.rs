use crate::cli::RequestedApp;

#[derive(Debug, Default, PartialEq)]
pub struct Config(pub Vec<RequestedApp>);

impl Config {
    pub fn lookup(self, app_name: &str) -> Option<RequestedApp> {
        for requested_app in self.0 {
            if &requested_app.name == app_name {
                return Some(requested_app);
            }
        }
        None
    }
}
