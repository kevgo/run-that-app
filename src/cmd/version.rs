use std::process::ExitCode;

pub fn version() -> ExitCode {
    println!(env!("CARGO_PKG_VERSION"));
    ExitCode::SUCCESS
}
