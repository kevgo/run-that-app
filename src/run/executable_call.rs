use super::executable_path::add_paths;
use super::{exit_status_to_code, ExecutableArgs, ExecutablePath};
use crate::{cli, prelude::*};
use std::fmt::{Display, Write};
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;
use std::process::{self, Child, Command, ExitCode, Stdio};
use std::sync::mpsc;
use std::thread;

/// information to call an `App`s executable, as it is defined by the user
#[derive(Clone)]
pub(crate) struct ExecutableCallDefinition {
  pub(crate) executable_path: ExecutablePath,
  pub(crate) args: ExecutableArgs,
}

impl ExecutableCallDefinition {
  pub(crate) fn into_executable_call(self, app_folder: &Path) -> Option<ExecutableCall> {
    match self.args {
      ExecutableArgs::None => Some(ExecutableCall {
        executable_path: self.executable_path,
        args: vec![],
      }),
      ExecutableArgs::OneOfTheseInAppFolder { options } => {
        for option in options {
          let full_path = app_folder.join(option);
          if full_path.exists() {
            return Some(ExecutableCall {
              executable_path: self.executable_path,
              args: vec![full_path.to_string_lossy().to_string()],
            });
          }
        }
        None
      }
    }
  }
}

impl Display for ExecutableCallDefinition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.executable_path.as_str())?;
    f.write_str(&self.args.to_string())?;
    Ok(())
  }
}

/// information to call an app with file paths adjusted
pub(crate) struct ExecutableCall {
  pub(crate) executable_path: ExecutablePath,
  pub(crate) args: Vec<String>,
}

impl ExecutableCall {
  /// Executes the given executable with the given arguments.
  /// The returned `ExitCode` also indicates failure if there has been any output.
  #[allow(clippy::unwrap_used)]
  pub(crate) fn check_output(&self, args: &[String], apps_to_include: &[ExecutableCall]) -> Result<ExitCode> {
    let (sender, receiver) = mpsc::channel();
    let mut cmd = Command::new(&self.executable_path);
    cmd.args(&self.args);
    cmd.args(args);
    let mut paths_to_include = vec![self.executable_path.as_path().parent().unwrap()];
    for app_to_include in apps_to_include {
      paths_to_include.push(app_to_include.executable_path.as_path().parent().unwrap());
    }
    add_paths(&mut cmd, &paths_to_include);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    let mut process = cmd.spawn().map_err(|err| UserError::CannotExecuteBinary {
      call: self.format_with_extra_args(args),
      reason: err.to_string(),
    })?;
    let Some(stdout) = process.stdout.take() else {
      return Err(UserError::CannotOpenSubshellStream);
    };
    monitor_output(stdout, sender.clone());
    let Some(stderr) = process.stderr.take() else {
      return Err(UserError::CannotOpenSubshellStream);
    };
    monitor_output(stderr, sender.clone());
    monitor_exit(process, sender);
    let mut encountered_output = false;
    let mut exit_code = ExitCode::SUCCESS;
    let mut stdout = io::stdout();
    for event in receiver {
      match event {
        Event::PermanentLine(line) | Event::TempLine(line) => {
          encountered_output = true;
          let mut colored_line: Vec<u8> = Vec::with_capacity(line.len() + BASH_RED.len() + BASH_CLEAR.len());
          colored_line.extend(BASH_RED);
          colored_line.extend(&line);
          colored_line.extend(BASH_CLEAR);
          if let Err(err) = io::Write::write_all(&mut stdout, &colored_line) {
            eprintln!("Cannot print colored text: {err}");
          }
        }
        Event::UnterminatedLine(line) => {
          encountered_output = true;
          let mut colored_line: Vec<u8> = Vec::with_capacity(line.len() + BASH_RED.len() + BASH_CLEAR.len() + 1);
          colored_line.extend(BASH_RED);
          colored_line.extend(&line);
          colored_line.extend(BASH_CLEAR);
          colored_line.push(b'\n');
          if let Err(err) = io::Write::write_all(&mut stdout, &colored_line) {
            eprintln!("Cannot print colored text: {err}");
          }
        }
        Event::Ended { exit_status } => {
          exit_code = exit_status_to_code(exit_status);
          break;
        }
      }
    }
    if encountered_output {
      let mut call = vec![self.executable_path.as_path().file_name().unwrap_or_default().to_string_lossy().to_string()];
      call.extend(args.to_owned());
      return Err(UserError::ProcessEmittedOutput { cmd: call.join(" ") });
    }
    Ok(exit_code)
  }

  /// provides a printable version of this `ExecutableCall` when called with additional arguments
  pub(crate) fn format_with_extra_args(&self, args: &[String]) -> String {
    let mut result = self.to_string();
    for arg in args {
      result.push(' ');
      result.push_str(arg);
    }
    result
  }

  /// Runs the given executable with the given arguments.
  /// Streams output to the user's terminal.
  #[allow(clippy::unwrap_used)]
  pub(crate) fn stream_output(&self, args: &[String], apps_to_include: &[ExecutableCall]) -> Result<ExitCode> {
    let mut cmd = Command::new(&self.executable_path);
    cmd.args(&self.args);
    cmd.args(args);
    let mut paths_to_include = vec![self.executable_path.as_path().parent().unwrap()];
    for app_to_include in apps_to_include {
      paths_to_include.push(app_to_include.executable_path.as_path().parent().unwrap());
    }
    add_paths(&mut cmd, &paths_to_include);
    let exit_status = cmd.status().map_err(|err| UserError::CannotExecuteBinary {
      call: self.format_with_extra_args(args),
      reason: err.to_string(),
    })?;
    Ok(exit_status_to_code(exit_status))
  }
}

impl Display for ExecutableCall {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.executable_path.as_str())?;
    for arg in &self.args {
      f.write_char(' ')?;
      f.write_str(arg)?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::ExecutableCall;
  use crate::run::ExecutablePath;
  use big_s::S;
  use std::path::Path;

  mod stream_output {
    use crate::run::{ExecutableCall, ExecutablePath};
    use big_s::S;
    use std::fs;

    #[test]
    #[cfg(unix)]
    fn unix_success() {
      use std::io::Write;
      use std::os::unix::fs::PermissionsExt;
      let tempdir = tempfile::tempdir().unwrap();
      let executable_path = tempdir.path().join("executable");
      let mut file = fs::File::create(&executable_path).unwrap();
      file.write_all(b"#!/bin/sh\necho hello").unwrap();
      file.set_permissions(fs::Permissions::from_mode(0o744)).unwrap();
      file.flush().unwrap();
      drop(file);
      // NOTE: if the test is flaky, wait 10 ms here.
      let executable_call = ExecutableCall {
        executable_path: ExecutablePath::from(executable_path),
        args: vec![],
      };
      let have = executable_call.stream_output(&[], &[]).unwrap();
      // HACK: is there a better way to compare ExitCode?
      assert_eq!(format!("{have:?}"), S("ExitCode(unix_exit_status(0))"));
    }

    #[test]
    #[cfg(unix)]
    fn unix_error() {
      use crate::filesystem::make_file_executable;
      let tempdir = tempfile::tempdir().unwrap();
      let executable_path = tempdir.path().join("executable");
      fs::write(&executable_path, b"#!/bin/sh\nexit 3").unwrap();
      make_file_executable(&executable_path).unwrap();
      let executable_call = ExecutableCall {
        executable_path: ExecutablePath::from(executable_path),
        args: vec![],
      };
      let have = executable_call.stream_output(&[], &[]).unwrap();
      // HACK: is there a better way to compare ExitCode?
      assert_eq!(format!("{have:?}"), S("ExitCode(unix_exit_status(3))"));
    }

    #[test]
    #[cfg(windows)]
    fn windows_success() {
      let tempdir = tempfile::tempdir().unwrap();
      let executable_path = tempdir.path().join("executable.cmd");
      fs::write(&executable_path, b"echo hello").unwrap();
      let executable_call = ExecutableCall {
        executable_path: ExecutablePath::from(executable_path),
        args: vec![],
      };
      let have = stream_output(&executable_call, &[], &[]).unwrap();
      // HACK: is there a better way to compare ExitCode?
      assert_eq!(format!("{have:?}"), S("ExitCode(ExitCode(0))"));
    }

    #[test]
    #[cfg(windows)]
    fn windows_error() {
      let tempdir = tempfile::tempdir().unwrap();
      let executable_path = tempdir.path().join("executable.cmd");
      fs::write(&executable_path, b"EXIT 3").unwrap();
      let executable_call = ExecutableCall {
        executable_path: ExecutablePath::from(executable_path),
        args: vec![],
      };
      let have = stream_output(&executable_call, &[], &[]).unwrap();
      // HACK: is there a better way to compare ExitCode?
      assert_eq!(format!("{have:?}"), S("ExitCode(ExitCode(3))"));
    }
  }

  #[test]
  fn format_with_extra_args() {
    let call = ExecutableCall {
      executable_path: ExecutablePath::from(Path::new("executable")),
      args: vec![S("arg1"), S("arg2")],
    };
    let have = call.format_with_extra_args(&[S("arg3")]);
    let want = S("executable arg1 arg2 arg3");
    assert_eq!(have, want);
  }

  #[test]
  fn to_string() {
    let call = ExecutableCall {
      executable_path: ExecutablePath::from(Path::new("executable")),
      args: vec![S("arg1"), S("arg2")],
    };
    let have = call.to_string();
    let want = S("executable arg1 arg2");
    assert_eq!(have, want);
  }
}

/// escape sequence to print red output on the shell
const BASH_RED: &[u8] = "\x1B[0;31m".as_bytes();
/// escape sequence to reset the output color on the shell
const BASH_CLEAR: &[u8] = "\x1B[0m".as_bytes();

/// events that can happen with subshells
pub(crate) enum Event {
  /// a line of output to STDOUT or STDERR terminated by LF
  PermanentLine(Vec<u8>),
  /// a line of output to STDOUT or STDERR terminated by CR
  TempLine(Vec<u8>),
  /// a line of output to STDOUT or STDERR without a CR or LF at the end
  UnterminatedLine(Vec<u8>),
  /// the process has ended with the given exit code
  Ended { exit_status: process::ExitStatus },
}

/// starts a thread that monitors the given STDOUT or STDERR stream
fn monitor_output<R: 'static + Read + Send>(stream: R, sender: mpsc::Sender<Event>) {
  let mut reader = BufReader::new(stream);
  thread::spawn(move || loop {
    let buffer = match reader.fill_buf() {
      Ok(buffer) => buffer,
      Err(err) => cli::exit(format!("cannot write subshell output into buffer: {err}")),
    };
    if buffer.is_empty() {
      break;
    }
    let consumed = buffer.iter().take_while(|c| **c != b'\n' && **c != b'\x0D').count();
    let total = if consumed < buffer.len() {
      // stopped at one of the EOL characters
      consumed + 1
    } else {
      // found no EOL character
      consumed
    };
    let line = buffer[0..total].to_owned();
    reader.consume(total);
    let event = match line.get(consumed) {
      Some(b'\n') => Event::PermanentLine(line),
      Some(b'\x0D') => Event::TempLine(line),
      _ => Event::UnterminatedLine(line),
    };
    if let Err(err) = sender.send(event) {
      eprintln!("cannot send subshell output through internal pipe: {err}");
    }
  });
}

/// starts the thread that monitors for process exit
fn monitor_exit(mut process: Child, sender: mpsc::Sender<Event>) {
  thread::spawn(move || {
    let exit_status = process.wait().unwrap_or_default();
    if let Err(err) = sender.send(Event::Ended { exit_status }) {
      cli::exit(format!("cannot send exit signal through internal pipe: {err}"));
    }
  });
}
