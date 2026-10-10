use std::path::Path;
use std::process::Command;

/// provides a `Command` instance that runs the given shell script with the given arguments
#[cfg(not(windows))]
pub fn shell_script_call(shell_script: &Path, args: &[String]) -> Command {
  // `sh -c <script> <args>` parses only <script> as the command string.
  // The following argv entries become the shell's $0, $1, ... and are not
  // passed to the script. `exec "$0" "$@"` runs the script path in $0
  // and forwards every remaining argument to it, including values that
  // contain spaces or shell metacharacters.
  let mut command = Command::new("sh");
  command.arg("-c");
  let mut shell_args = Vec::with_capacity(args.len() + 1);
  shell_args.push(shell_script.to_string_lossy().to_string());
  shell_args.extend(args.iter().cloned());
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

#[cfg(test)]
mod tests {
  use super::shell_script_call;
  use crate::executables::cmd_to_string;
  use std::fs;
  #[cfg(not(windows))]
  use std::os::unix::fs::PermissionsExt;
  use std::path::{Path, PathBuf};

  /// Prints the argument count, then the first two arguments, each on its own line.
  #[cfg(windows)]
  const PRINT_TWO_ARGS: &str = "@echo off\r\n\
setlocal EnableExtensions EnableDelayedExpansion\r\n\
set \"arg1=%~1\"\r\n\
set \"arg2=%~2\"\r\n\
set n=0\r\n\
:loop\r\n\
if \"%~1\"==\"\" goto print\r\n\
set /a n+=1\r\n\
shift\r\n\
goto loop\r\n\
:print\r\n\
echo !n!\r\n\
echo !arg1!\r\n\
echo !arg2!\r\n";
  #[cfg(not(windows))]
  const PRINT_TWO_ARGS: &str = "#!/bin/sh\nprintf '%s\\n' \"$#\" \"$1\" \"$2\"\n";

  /// Prints only the argument count, without a trailing newline.
  #[cfg(windows)]
  const PRINT_ARG_COUNT: &str = "@echo off\r\n\
setlocal EnableExtensions\r\n\
set n=0\r\n\
:loop\r\n\
if \"%~1\"==\"\" goto print\r\n\
set /a n+=1\r\n\
shift\r\n\
goto loop\r\n\
:print\r\n\
<nul set /p =%n%\r\n\
exit /b 0\r\n";
  #[cfg(not(windows))]
  const PRINT_ARG_COUNT: &str = "#!/bin/sh\nprintf '%s' \"$#\"\n";

  #[cfg(windows)]
  const SCRIPT_NAME: &str = "args.cmd";
  #[cfg(not(windows))]
  const SCRIPT_NAME: &str = "args.sh";

  fn write_script(dir: &Path, name: &str, body: &str) -> PathBuf {
    let path = dir.join(name);
    fs::write(&path, body).unwrap();
    #[cfg(not(windows))]
    let mut permissions = fs::metadata(&path).unwrap().permissions();
    #[cfg(not(windows))]
    permissions.set_mode(0o755);
    #[cfg(not(windows))]
    fs::set_permissions(&path, permissions).unwrap();
    path
  }

  fn assert_script_output(output: &std::process::Output, want: &str) {
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    // `cmd` prints CRLF. The script's logical output uses LF on every platform.
    let have = String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n");
    assert_eq!(have, want);
  }

  /// `cmd_to_string` renders the `cmd /C` invocation with POSIX quoting.
  #[cfg(windows)]
  fn windows_command(script: &Path, rendered_args: &str) -> String {
    let script = script.to_string_lossy();
    #[allow(clippy::unwrap_used)]
    let quoted_script = shlex::try_quote(script.as_ref()).unwrap();
    format!("cmd /C {quoted_script} {rendered_args}")
  }

  #[test]
  fn passes_arguments_to_the_script() {
    let dir = tempfile::tempdir().unwrap();
    let script = write_script(dir.path(), SCRIPT_NAME, PRINT_TWO_ARGS);
    let mut cmd = shell_script_call(&script, &["--version".to_string(), "hello world".to_string()]);
    let have = cmd_to_string(&cmd);
    #[cfg(windows)]
    let want = windows_command(&script, "--version 'hello world'");
    #[cfg(not(windows))]
    let want = format!(r#"sh -c "{} --version 'hello world'""#, script.display());
    assert_eq!(have, want);

    assert_script_output(&cmd.output().unwrap(), "2\n--version\nhello world\n");
  }

  #[test]
  fn encodes_special_characters() {
    let dir = tempfile::tempdir().unwrap();
    let script = write_script(dir.path(), SCRIPT_NAME, PRINT_TWO_ARGS);
    let mut cmd = shell_script_call(&script, &["--version".to_string(), "hello $HOME".to_string()]);
    let have = cmd_to_string(&cmd);
    #[cfg(windows)]
    let want = windows_command(&script, "--version 'hello $HOME'");
    #[cfg(not(windows))]
    let want = format!(r#"sh -c "{} --version 'hello "'$HOME'"'""#, script.display());
    assert_eq!(have, want);

    assert_script_output(&cmd.output().unwrap(), "2\n--version\nhello $HOME\n");
  }

  #[test]
  fn passes_no_arguments() {
    let dir = tempfile::tempdir().unwrap();
    let script = write_script(dir.path(), SCRIPT_NAME, PRINT_ARG_COUNT);
    assert_script_output(&shell_script_call(&script, &[]).output().unwrap(), "0");
  }
}
