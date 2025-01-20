use super::ExecutablePath;

/// provides a printable version of the given executable invocation
// TODO: move into the upcoming CallSignature
pub fn format_call(executable: &ExecutablePath, args: &[String]) -> String {
  format!("{} {}", executable.as_path().to_string_lossy(), args.join(" "))
}
