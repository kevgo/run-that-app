use super::ExecutableCall;

/// provides a printable version of the given executable invocation
// TODO: move into the upcoming CallSignature
pub fn format_call(executable_call: &ExecutableCall, args: &[String]) -> String {
  let mut result = String::from(executable_call.executable_path.as_str());
  for arg in &executable_call.args {
    result.push(' ');
    result.push_str(arg);
  }
  for arg in args {
    result.push(' ');
    result.push_str(arg);
  }
  result
}
