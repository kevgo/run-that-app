use crate::ui::RequestedApp;

use super::RunnableApp;

pub fn load_runnable_app(requested_app: &RequestedApp) -> Option<RunnableApp> {
    None
}

#[cfg(test)]
mod tests {

    #[test]
    fn app_in_installed() {
        // TODO
    }

    #[test]
    fn app_is_not_installed() {
        // TODO
    }

    #[test]
    fn app_is_installed_but_wrong_version() {
        // TODO
    }
}
