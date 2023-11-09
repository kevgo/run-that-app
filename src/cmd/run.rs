use crate::apps;
use crate::archives;
use crate::detect;
use crate::subshell;
use crate::ui::{Output, RequestedApp};
use crate::yard;
use crate::yard::RunnableApp;
use crate::Result;

pub fn run(requested_app: RequestedApp, output: &dyn Output) -> Result<()> {
    let prodyard = yard::production()?;
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
    let online_location = app.online_location(requested_app.version.clone(), &platform);
    let artifact = online_location.download(output)?;
    let archive = archives::lookup(artifact);
    archive.extract(
        app.file_to_extract_from_archive(&requested_app.version, &platform),
        prodyard.file_path(requested_app, app.executable(&platform)),
        output,
    )
}
