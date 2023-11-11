use crate::apps;
use crate::apps::App;
use crate::archives;
use crate::cli::RequestedApp;
use crate::detect;
use crate::detect::Platform;
use crate::subshell;
use crate::yard;
use crate::yard::Executable;
use crate::yard::Yard;
use crate::Output;
use crate::Result;
use std::process::ExitCode;

pub fn run(
    requested_app: &RequestedApp,
    args: Vec<String>,
    output: &dyn Output,
) -> Result<ExitCode> {
    let app = apps::lookup(&requested_app.name)?;
    let platform = detect::detect(output)?;
    let prodyard = yard::load_or_create(&yard::production_location()?)?;
    let executable = match prodyard.load_app(requested_app, app.executable(platform)) {
        Some(installed_app) => installed_app,
        None => install_app(requested_app, app.as_ref(), platform, &prodyard, output)?,
    };
    Ok(subshell::execute(executable, args))
}

fn install_app(
    requested_app: &RequestedApp,
    known_app: &dyn App,
    platform: Platform,
    prodyard: &Yard,
    output: &dyn Output,
) -> Result<Executable> {
    let online_location = known_app.artifact_location(&requested_app.version, platform);
    let artifact = online_location.download(output)?;
    prodyard.create_app_folder(requested_app)?;
    archives::extract(
        artifact,
        known_app.file_to_extract_from_archive(&requested_app.version, platform),
        prodyard.app_file_path(requested_app, known_app.executable(platform)),
        output,
    )
}
