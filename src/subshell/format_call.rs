use super::Executable;

/// provides a printable version of the given executable invocation
pub fn format_call(executable: &Executable, args: &[String]) -> String {
  format!("{} {}", executable.0.to_string_lossy(), args.join(" "))
}
