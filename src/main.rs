use std::process::ExitCode;

fn main() -> ExitCode {
  rta::run_or_exit(std::env::args().skip(1));
  rta::commands::run();
}
