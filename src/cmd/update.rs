use std::process::ExitCode;

use crate::output::Output;

pub fn update(output: &dyn Output) -> ExitCode {
    output.println("updating");
    ExitCode::SUCCESS
}
