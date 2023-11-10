use crate::Output;
use std::process::ExitCode;

pub fn version(output: &dyn Output) -> ExitCode {
    output.println(env!("CARGO_PKG_VERSION"));
    ExitCode::SUCCESS
}
