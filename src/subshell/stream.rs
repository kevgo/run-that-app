use super::{call_signature, exit_status_to_code};
use crate::error::UserError;
use crate::subshell::Executable;
use crate::Result;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process::{self, Child, Command, ExitCode, Stdio};
use std::sync::mpsc;
use std::thread;

/// events that can happen with subshells
pub enum Event {
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

/// Executes the given executable with the given arguments.
/// The returned `ExitCode` also indicates failure if there has been any output.
pub fn stream(executable: &Executable, args: &[String]) -> Result<ExitCode> {
    let (sender, receiver) = mpsc::channel();
    let mut cmd = Command::new(&executable.0);
    cmd.args(args);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    let mut process = cmd.spawn().map_err(|err| UserError::CannotExecuteBinary {
        call: call_signature(executable, args),
        reason: err.to_string(),
    })?;
    monitor_output(process.stdout.take().unwrap(), sender.clone());
    monitor_output(process.stderr.take().unwrap(), sender.clone());
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
                stdout.write_all(&colored_line).unwrap();
            }
            Event::UnterminatedLine(line) => {
                encountered_output = true;
                let mut colored_line: Vec<u8> = Vec::with_capacity(line.len() + BASH_RED.len() + BASH_CLEAR.len() + 1);
                colored_line.extend(BASH_RED);
                colored_line.extend(&line);
                colored_line.extend(BASH_CLEAR);
                colored_line.push(b'\n');
                stdout.write_all(&colored_line).unwrap();
            }
            Event::Ended { exit_status } => {
                exit_code = exit_status_to_code(exit_status);
                break;
            }
        }
    }
    if encountered_output {
        let mut call = vec![executable.0.file_name().unwrap_or_default().to_string_lossy().to_string()];
        call.extend(args.to_owned());
        return Err(UserError::ProcessEmittedOutput { cmd: call.join(" ") });
    }
    Ok(exit_code)
}

/// starts a thread that monitors the given STDOUT or STDERR stream
fn monitor_output<R: 'static + Read + Send>(stream: R, sender: mpsc::Sender<Event>) {
    let mut reader = BufReader::new(stream);
    thread::spawn(move || loop {
        let buffer = reader.fill_buf().unwrap();
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
        sender.send(event).unwrap();
    });
}

/// starts the thread that monitors for process exit
fn monitor_exit(mut process: Child, sender: mpsc::Sender<Event>) {
    thread::spawn(move || {
        let exit_status = process.wait().unwrap();
        sender.send(Event::Ended { exit_status }).unwrap();
    });
}
