use crate::config;
use crate::output::Output;
use crate::Result;
use std::process::ExitCode;

pub fn update(output: &dyn Output) -> Result<ExitCode> {
    let config = config::load()?;
    output.println("updating");
    Ok(ExitCode::SUCCESS)
}
