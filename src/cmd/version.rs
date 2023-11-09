use crate::Output;
use std::process::ExitCode;

pub fn version(output: &dyn Output) -> ExitCode {
    output.print("version");
    ExitCode::SUCCESS
}
