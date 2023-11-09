use crate::yard::RunnableApp;
use std::process::{Command, ExitCode};

pub fn execute(app: RunnableApp, args: Vec<String>) -> ExitCode {
    let mut cmd = Command::new(app.executable);
    cmd.args(args);
    let exit_status = cmd.status().unwrap();
    let exit_code = exit_status.code().unwrap_or(255);
    ExitCode::from(reduce_i32_to_u8(exit_code))
}

fn reduce_i32_to_u8(code: i32) -> u8 {
    if code < 0 {
        return 255;
    }
    if code > 255 {
        return 255;
    }
    u8::try_from(code).unwrap()
}
