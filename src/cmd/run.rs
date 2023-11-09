use crate::apps;
use crate::apps::App;
use crate::archives;
use crate::detect;
use crate::detect::Platform;
use crate::subshell;
use crate::ui::{Output, RequestedApp};
use crate::yard;
use crate::yard::RunnableApp;
use crate::yard::Yard;
use crate::Result;

pub fn run(requested_app: RequestedApp, output: &dyn Output) -> Result<()> {
    let app = apps::lookup(&requested_app.name)?;
    let platform = detect::detect(output)?;
    let prodyard = yard::load(yard::production_location())?;
    let runnable_app = match prodyard.load(&requested_app, app.executable(&platform)) {
        Some(installed_app) => installed_app,
        None => install_app(&requested_app, app, &platform, prodyard, output)?,
    };
    subshell::execute(runnable_app)
}

fn install_app(
    requested_app: &RequestedApp,
    app: Box<dyn App>,
    platform: &Platform,
    prodyard: Yard,
    output: &dyn Output,
) -> Result<RunnableApp> {
    let online_location = app.online_location(requested_app.version.clone(), &platform);
    let artifact = online_location.download(output)?;
    let archive = archives::lookup(artifact);
    archive.extract(
        app.file_to_extract_from_archive(&requested_app.version, &platform),
        prodyard.file_path(requested_app, app.executable(&platform)),
        output,
    )
}
