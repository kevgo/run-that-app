use std::process::Command;

/// provides a human-readable string representation of the given command
pub fn cmd_to_string(cmd: &Command) -> String {
  let mut result = String::new();
  result.push_str(&cmd.get_program().to_string_lossy());
  for arg in cmd.get_args() {
    result.push(' ');
    result.push_str(&arg.to_string_lossy());
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cmd_to_string() {
    let mut cmd = Command::new("echo");
    cmd.arg("Hello, world!");
    assert_eq!(cmd_to_string(&cmd), "echo Hello, world!");
  }
}
