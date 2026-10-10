use std::process::Command;

/// provides a human-readable string representation of the given command
pub fn cmd_to_string(cmd: &Command) -> String {
  let args = cmd.get_args();
  let mut pieces = Vec::with_capacity(args.len() + 1);
  pieces.push(cmd.get_program().to_string_lossy().to_string());
  pieces.extend(args.into_iter().map(|arg| arg.to_string_lossy().to_string()));
  #[allow(clippy::unwrap_used)]
  shlex::try_join(pieces.iter().map(std::string::String::as_str)).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cmd_to_string() {
    let mut cmd = Command::new("echo");
    cmd.arg("Hello, world");
    cmd.arg("!");
    assert_eq!(cmd_to_string(&cmd), "echo 'Hello, world' '!'");
  }
}
