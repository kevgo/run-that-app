use std::process::ExitCode;

fn main() -> ExitCode {
  let actionlint = rta::applications::ActionLint {};
  let apps = rta::applications::all();
  let cmd = rta::get_cmd(
    &actionlint,
    rta::GetCmdArgs {
      version: Some("1.7.12".into()),
      app_args: vec!["--help".into()],
      error_on_output: false,
      from_source: false,
      include_apps: vec![],
      optional: false,
      verbose: true,
    },
    &apps,
  );
  let mut cmd = cmd.unwrap().unwrap();
  let exit_code = cmd.status().unwrap();
  match rta::run(std::env::args().skip(1)) {
    Ok(exitcode) => exitcode,
    Err(err) => {
      err.print();
      ExitCode::FAILURE
    }
  }
}
