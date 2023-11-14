use crate::apps;
use crate::apps::App;
use crate::cli::RequestedApp;
use crate::detect;
use crate::detect::Platform;
use crate::error::UserError;
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
    let executable = load_or_install(requested_app, app.as_ref(), platform, &prodyard, output)?;
    Ok(subshell::execute(executable, args))
}

fn load_or_install(
    requested_app: &RequestedApp,
    app: &dyn App,
    platform: Platform,
    yard: &Yard,
    output: &dyn Output,
) -> Result<Executable> {
    if let Some(executable) = yard.load_app(requested_app, app.executable(platform)) {
        return Ok(executable);
    };
    for installation_method in app.installation_methods(&requested_app.version, platform, yard) {
        if let Some(executable) = installation_method.install(output)? {
            return Ok(executable);
        }
    }
    Err(UserError::UnsupportedPlatform)
}
