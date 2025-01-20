use super::ExecutablePath;

/// a way to call an executable
pub struct ExecutableCall {
  /// the executable to call
  pub executable: ExecutablePath,
  /// arguments that are part of running the executable itself, not arguments provided by the user
  pub args: Vec<&'static str>,
}
