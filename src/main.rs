use std::process::ExitCode;

fn main() -> ExitCode {
  match rta::run(std::env::args().skip(1)) {
    Ok(exitcode) => exitcode,
    Err(err) => {
      err.print();
      ExitCode::FAILURE
    }
  }
}
