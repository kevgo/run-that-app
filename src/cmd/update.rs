use crate::cli::RequestedApp;
use crate::output::Output;
use crate::Result;
use crate::{apps, config};
use std::process::ExitCode;

pub fn update(output: &dyn Output) -> Result<ExitCode> {
    output.println("updating");
    let config = config::load()?;
    let mut new_config: Vec<RequestedApp> = vec![];
    let apps = apps::all();
    for configured_app in &config.0 {
        let Some(app) = apps.iter().find(|&app| app.name() == configured_app.name) else {
            continue;
        };
        output.println(&format!("updating {} ...", app.name()));
    }
    // write new_config to disk
    Ok(ExitCode::SUCCESS)
}
