use crate::apps;
use crate::detect;
use crate::subshell;
use crate::ui::RequestedApp;
use crate::yard;
use crate::{Output, Result};

pub fn run(requested_app: RequestedApp, output: &Output) -> Result<()> {
    let platform = detect::detect(output)?;
    let prodyard = yard::production_instance()?;
    let app = apps::lookup(&requested_app.name)?;
    let runnable_app = match prodyard.load(&requested_app) {
        Some(installed_app) => installed_app,
        None => {
            let hoster = app.hoster();
            let artifact = hoster.download(&platform)?;
            let archive = artifact.to_archive()?;
            archive.extract(
                app.files_to_extract_from_archive(&requested_app.version),
                &prodyard.folder_for(&requested_app),
            )?;
            prodyard.load(&requested_app).unwrap()
        }
    };
    subshell::execute(runnable_app)
}
