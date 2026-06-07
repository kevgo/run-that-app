use std::process::ExitCode;

use rta::applications::AppDefinition;

fn main() -> ExitCode {
  let gh = rta::applications::Gh {};
  let name = gh.name();
  match rta::run(std::env::args().skip(1)) {
    Ok(exitcode) => exitcode,
    Err(err) => {
      err.print();
      ExitCode::FAILURE
    }
  }
}
