use std::process::ExitCode;

fn main() -> ExitCode {
  rta::commands::help();
  rta::run_or_exit(std::env::args().skip(1))
}
