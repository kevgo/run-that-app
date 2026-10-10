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
  let mut shell_args = Vec::with_capacity(app_args.len() + 1);
  shell_args.push(shell_script.to_string_lossy().to_string());
  shell_args.extend(app_args.iter().cloned());
  #[allow(clippy::unwrap_used)]
  let script_args = shlex::try_join(shell_args.iter().map(std::string::String::as_str)).unwrap();
  command.arg(script_args);
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
  use crate::executables::cmd_to_string;
  use std::fs;
  use std::os::unix::fs::PermissionsExt;
  use std::path::{Path, PathBuf};

  fn write_script(dir: &Path, name: &str, body: &str) -> PathBuf {
    let path = dir.join(name);
    #[allow(clippy::unwrap_used)]
    fs::write(&path, body).unwrap();
    #[allow(clippy::unwrap_used)]
    let mut permissions = fs::metadata(&path).unwrap().permissions();
    permissions.set_mode(0o755);
    #[allow(clippy::unwrap_used)]
    fs::set_permissions(&path, permissions).unwrap();
    path
  }

  #[test]
  fn passes_arguments_to_the_script() {
    let dir = tempfile::tempdir().unwrap();
    let script = write_script(dir.path(), "args.sh", "#!/bin/sh\nprintf '%s\\n' \"$#\" \"$1\" \"$2\"\n");
    let mut cmd = shell_script_call(&script, &["--version".to_string(), "hello world".to_string()]);
    let have = cmd_to_string(&cmd);
    let want = format!(r#"sh -c "{} --version 'hello world'""#, script.to_string_lossy());
    assert_eq!(have, want);

    let output = cmd.output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(str::from_utf8(&output.stdout).unwrap(), "2\n--version\nhello world\n");
  }

  #[test]
  fn encodes_special_characters() {
    let dir = tempfile::tempdir().unwrap();
    let script = write_script(dir.path(), "args.sh", "#!/bin/sh\nprintf '%s\\n' \"$#\" \"$1\" \"$2\"\n");
    let mut cmd = shell_script_call(&script, &["--version".to_string(), "hello $HOME".to_string()]);
    let have = cmd_to_string(&cmd);
    let want = format!(r#"sh -c "{} --version 'hello "'$HOME'"'""#, script.to_string_lossy());
    assert_eq!(have, want);

    let output = cmd.output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(str::from_utf8(&output.stdout).unwrap(), "2\n--version\nhello $HOME\n");
  }

  #[test]
  fn passes_no_arguments() {
    let dir = tempfile::tempdir().unwrap();
    let script = write_script(dir.path(), "args.sh", "#!/bin/sh\nprintf '%s' \"$#\"\n");
    let output = shell_script_call(&script, &[]).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(str::from_utf8(&output.stdout).unwrap(), "0");
  }
}
