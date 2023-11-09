use crate::yard::RunnableApp;
use crate::Result;
use std::process::{Command, ExitCode};

pub fn execute(app: RunnableApp, args: Vec<String>) -> Result<ExitCode> {
    let mut cmd = Command::new(app.executable);
    cmd.args(args);
    let status = cmd.status().unwrap();
    let code = status.code().unwrap_or(0) as u8;
    Ok(ExitCode::from(code))
}
