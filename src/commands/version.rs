use std::process::ExitCode;

pub(crate) fn version() -> ExitCode {
  println!(env!("CARGO_PKG_VERSION"));
  ExitCode::SUCCESS
}
