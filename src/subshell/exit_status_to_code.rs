use std::process::{ExitCode, ExitStatus};

pub fn exit_status_to_code(exit_status: ExitStatus) -> ExitCode {
  if exit_status.success() {
    return ExitCode::SUCCESS;
  }
  let Some(big_code) = exit_status.code() else {
    return ExitCode::FAILURE;
  };
  match u8::try_from(big_code) {
    Ok(small_code) => ExitCode::from(small_code),
    Err(_) => ExitCode::from(255),
  }
}
