use std::process::ExitCode;

#[must_use]
pub fn version() -> ExitCode {
  println!(env!("CARGO_PKG_VERSION"));
  ExitCode::SUCCESS
}
