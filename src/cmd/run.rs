use crate::apps;
use crate::detect;
use crate::subshell;
use crate::ui::RequestedApp;
use crate::yard;
use crate::yard::RunnableApp;
use crate::{Output, Result};

pub fn run(requested_app: RequestedApp, output: &Output) -> Result<()> {
    let prodyard = yard::production_instance()?;
    let runnable_app = match prodyard.load(&requested_app) {
        Some(installed_app) => installed_app,
        None => install_app(requested_app, prodyard, output)?,
    };
    subshell::execute(runnable_app)
}

fn install_app(
    requested_app: RequestedApp,
    prodyard: yard::Yard,
    output: &Output,
) -> Result<RunnableApp> {
    let app = apps::lookup(&requested_app.name)?;
    let platform = detect::detect(output)?;
    let hoster = app.hoster();
    let artifact = hoster.download(&platform)?;
    let archive = artifact.to_archive();
    archive.extract(
        app.files_to_extract_from_archive(&requested_app.version),
        &prodyard.folder_for(&requested_app),
    )
}
