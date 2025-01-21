use super::ExecutableCall;

/// provides a printable version of the given executable invocation
// TODO: move into the upcoming CallSignature
pub fn format_call(executable_call: &ExecutableCall, args: &[String]) -> String {
  format!(
    "{} {} {}",
    executable_call.executable_path.as_path().to_string_lossy(),
    executable_call.args.join(" "),
    args.join(" ")
  )
}
