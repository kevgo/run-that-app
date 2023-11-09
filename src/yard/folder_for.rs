use crate::ui::RequestedApp;
use std::path::PathBuf;

pub fn folder_for(app: &RequestedApp) -> PathBuf {
    PathBuf::new()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::folder_for;
    use crate::ui::RequestedApp;
    use big_s::S;

    #[test]
    fn foo() {
        let app = RequestedApp {
            name: S("shellcheck"),
            version: S("0.9.0"),
        };
        let have = folder_for(&app);
        let want = PathBuf::from("");
    }
}
