use super::{add_paths, exit_status_to_code};
use crate::cli;
use crate::prelude::*;
use crate::run::ExecutableCall;
use std::io::{self, BufRead, BufReader, Read};
use std::process::{self, Child, Command, ExitCode, Stdio};
use std::sync::mpsc;
use std::thread;

/// Executes the given executable with the given arguments.
/// The returned `ExitCode` also indicates failure if there has been any output.
#[allow(clippy::unwrap_used)]
pub(crate) fn copy_output(executable: &ExecutableCall, args: &[String], apps_to_include: &[ExecutableCall]) -> Result<(bool, ExitCode)> {
  let (sender, receiver) = mpsc::channel();
  let mut cmd = Command::new(&executable.executable_path);
  cmd.args(&executable.args);
  cmd.args(args);
  let mut paths_to_include = vec![executable.executable_path.as_path().parent().unwrap()];
  for app_to_include in apps_to_include {
    paths_to_include.push(app_to_include.executable_path.as_path().parent().unwrap());
  }
  add_paths(&mut cmd, &paths_to_include);
  cmd.stdout(Stdio::piped());
  cmd.stderr(Stdio::piped());
  let mut process = cmd.spawn().map_err(|err| UserError::CannotExecuteBinary {
    call: executable.format_with_extra_args(args),
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
  Ok((encountered_output, exit_code))
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

/// escape sequence to print red output on the shell
const BASH_RED: &[u8] = "\x1B[0;31m".as_bytes();
/// escape sequence to reset the output color on the shell
const BASH_CLEAR: &[u8] = "\x1B[0m".as_bytes();
