use crate::error::UserError;
use crate::yard::Executable;
use crate::Result;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process::{self, Child, Command, ExitCode, ExitStatus, Stdio};
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

/// Starts the given Command instance in a separate thread.
/// Signals activity (output, finished) using the given MPSC sender.
pub fn stream(Executable(app): Executable, args: Vec<String>) -> Result<ExitCode> {
    let (sender, receiver) = mpsc::channel();
    let mut cmd = Command::new(&app);
    cmd.args(args);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    let mut process = cmd.spawn().map_err(|err| UserError::CannotExecuteBinary {
        executable: app,
        reason: err.to_string(),
    })?;
    monitor_output(process.stdout.take().unwrap(), sender.clone());
    monitor_output(process.stderr.take().unwrap(), sender.clone());
    monitor_exit(process, sender);
    let mut exit_code = ExitCode::SUCCESS;
    for event in receiver {
        match event {
            Event::PermanentLine(line) | Event::TempLine(line) => {
                exit_code = ExitCode::FAILURE;
                let _ = io::stdout().write_all(BASH_RED);
                io::stdout().write_all(&line).unwrap();
                let _ = io::stdout().write_all(BASH_CLEAR);
            }
            Event::UnterminatedLine(line) => {
                exit_code = ExitCode::FAILURE;
                io::stdout().write_all(&line).unwrap();
                println!();
            }
            Event::Ended { exit_status } => {
                exit_code = exit_status_to_code(exit_status);
                break;
            }
        }
    }
    Ok(exit_code)
}

fn exit_status_to_code(exit_status: ExitStatus) -> ExitCode {
    let Some(exit_status) = exit_status.code() else {
        return ExitCode::SUCCESS;
    };
    if exit_status == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
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
