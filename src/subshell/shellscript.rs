use std::path::Path;
use std::process::Command;

/// Runs `shell_script` through `sh`, with `app_args` as the script's own arguments.
#[cfg(not(windows))]
pub fn shell_script_call(shell_script: &Path, app_args: &[String]) -> Command {
  // `sh -c <script> <args>` parses only <script> as the command string.
  // The following argv entries become the shell's $0, $1, ... and are not
  // passed to the script. `exec "$0" "$@"` runs the script path in $0
  // and forwards every remaining argument to it, including values that
  // contain spaces or shell metacharacters.
  let mut command = Command::new("sh");
  command.arg("-c");
  command.arg(r#"exec "$0" "$@""#);
  command.arg(shell_script);
  command.args(app_args);
  command
}

/// Runs `shell_script` through `cmd`, with `app_args` as the script's own arguments.
#[cfg(windows)]
pub fn shell_script_call(shell_script: &Path, app_args: &[String]) -> Command {
  let mut args = Vec::with_capacity(app_args.len() + 1);
  args.push(shell_script.to_string_lossy().to_string());
  args.extend(app_args.iter().cloned());
  let mut command = Command::new("cmd");
  command.arg("/C");
  command.args(args);
  command
}

#[cfg(all(test, unix))]
mod tests {
  use super::shell_script_call;
  use std::os::unix::fs::PermissionsExt;
  use std::path::{Path, PathBuf};
  use std::{fs, io};

  fn write_script(dir: &Path, name: &str, body: &str) -> io::Result<PathBuf> {
    let path = dir.join(name);
    fs::write(&path, body)?;
    let mut permissions = fs::metadata(&path)?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&path, permissions)?;
    Ok(path)
  }

  /// `sh -c <script> <args>` would report zero script arguments here.
  #[test]
  fn passes_arguments_to_the_script() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let script = write_script(dir.path(), "args.sh", "#!/bin/sh\nprintf '%s\\n' \"$#\" \"$1\" \"$2\"\n")?;
    let output = shell_script_call(&script, &["--version".to_string(), "hello $HOME".to_string()]).output()?;
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(output.stdout, b"2\n--version\nhello $HOME\n");
    Ok(())
  }

  #[test]
  fn passes_no_arguments() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let script = write_script(dir.path(), "args.sh", "#!/bin/sh\nprintf '%s\\n' \"$#\"\n")?;
    let output = shell_script_call(&script, &[]).output()?;
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(output.stdout, b"0\n");
    Ok(())
  }
}
