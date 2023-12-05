use crate::cli::RequestedApp;
use crate::config;
use crate::output::Output;
use crate::Result;
use std::process::ExitCode;

pub fn update(output: &dyn Output) -> Result<ExitCode> {
    output.println("updating");
    let config = config::load()?;
    let mut new_config: Vec<RequestedApp> = vec![];
    for app in &config.0 {
        output.println(&format!("updating {} ...", app.name));
    }
    // write new_config to disk
    Ok(ExitCode::SUCCESS)
}
