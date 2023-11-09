use crate::Output;
use crate::Result;
use std::process::ExitCode;

pub fn version(output: &dyn Output) -> Result<ExitCode> {
    output.print("version");
    Ok(ExitCode::SUCCESS)
}
