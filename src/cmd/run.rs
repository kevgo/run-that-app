use crate::apps;
use crate::detect;
use crate::subshell;
use crate::ui::{Output, RequestedApp};
use crate::yard;
use crate::yard::RunnableApp;
use crate::Result;

pub fn run(requested_app: RequestedApp, output: &dyn Output) -> Result<()> {
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
    output: &dyn Output,
) -> Result<RunnableApp> {
    let app = apps::lookup(&requested_app.name)?;
    let platform = detect::detect(output)?;
    let online_asset = app.online_asset(requested_app.version.clone(), &platform);
    let artifact = online_asset.download(output)?;
    let archive = artifact.to_archive();
    archive.extract(
        app.file_to_extract_from_archive(&requested_app.version),
        &prodyard.folder_for(&requested_app),
        output,
    )
}
