use crate::detect;
use crate::hosting;
use crate::subshell;
use crate::ui::RequestedApp;
use crate::yard;
use crate::{Output, Result};

pub fn run(requested_app: RequestedApp, output: &Output) -> Result<()> {
    let platform = detect::detect(output)?;
    let runnable_app = if let Some(installed_app) = yard::load_runnable_app(&requested_app) {
        installed_app
    } else {
        let app_folder = yard::folder_for(&requested_app);
        hosting::download_app(&requested_app, &platform, app_folder)?
    };
    subshell::execute(runnable_app)
}
